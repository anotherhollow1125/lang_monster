use anyhow::Result;
use console::Alignment;
use console::Term;
use std::fmt;
use std::thread;
use std::time::Duration;

const PROMPT: char = '\u{25bc}';

// 枠線の参考: https://www.asahi-net.or.jp/~ax2s-kmtn/ref/unicode/u2500.html

#[derive(Debug, Clone)]
enum Line {
    None,
    Typing(String),
    Done(String),
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Line::None => write!(f, ""),
            Line::Typing(s) => write!(f, "{}", s),
            Line::Done(s) => write!(f, "{}", s),
        }
    }
}

impl Line {
    fn typing(&mut self, c: char) {
        match self {
            Line::None => *self = Line::Typing(c.to_string()),
            Line::Typing(s) => s.push(c),
            Line::Done(_) => {}
        }
    }

    fn into_done(&mut self) {
        match self {
            Line::None => {}
            Line::Typing(s) => {
                *self = Line::Done(s.clone());
            }
            Line::Done(_) => {}
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TypingStatus {
    Typing,
    Waiting,
    Done,
}

struct Lines<'a> {
    message_block: Vec<Vec<char>>,
    lines_inner: LinesInner,
    status: TypingStatus,
    picture: &'a str,
}

struct LinesInner {
    first: Line,
    second: Line,
}

impl LinesInner {
    fn target_line(&mut self) -> &mut Line {
        match (&self.first, &self.second) {
            (&Line::None, _) => &mut self.first,
            (&Line::Typing(_), _) => &mut self.first,
            (&Line::Done(_), &Line::None) => &mut self.second,
            (&Line::Done(_), &Line::Typing(_)) => &mut self.second,
            (&Line::Done(_), &Line::Done(_)) => {
                let second_line = self.second.clone();
                self.first = second_line;
                self.second = Line::None;
                &mut self.second
            }
        }
    }

    fn is_full(&self) -> bool {
        match (&self.first, &self.second) {
            (&Line::Done(_), &Line::Done(_)) => true,
            _ => false,
        }
    }
}

use TypingStatus as TS;

impl<'a> Lines<'a> {
    fn new(message_block: Vec<String>, picture: &'a str) -> Self {
        let mut message_block: Vec<Vec<char>> = message_block
            .into_iter()
            .map(|line| line.chars().rev().collect())
            .collect();
        message_block.reverse();
        Self {
            message_block,
            lines_inner: LinesInner {
                first: Line::None,
                second: Line::None,
            },
            status: TS::Typing,
            picture,
        }
    }

    fn prompt(&self) -> char {
        match self.status {
            TS::Typing => ' ',
            TS::Waiting => PROMPT,
            TS::Done => PROMPT,
        }
    }

    fn typing(&mut self) -> TS {
        let Lines {
            ref mut message_block,
            ref mut lines_inner,
            ..
        } = self;

        let Some(message) = message_block.last_mut() else {
            // reachable: message_block has only one line. (Because then lines_inner cannot become full.)
            self.status = TS::Done;
            return TS::Done;
        };

        let target_line = lines_inner.target_line();

        let c = message.pop();
        if let Some(c) = c {
            target_line.typing(c);
        }

        if message.is_empty() {
            message_block.pop();
            target_line.into_done();
        }

        self.status = match (lines_inner.is_full(), message_block.is_empty()) {
            (true, true) => TS::Done,
            (true, false) => TS::Waiting,
            _ => TS::Typing,
        };

        self.status
    }

    fn print(&self, term: &Term, show_prompt: bool, show_picture: bool) -> Result<()> {
        term.move_cursor_to(0, 0)?;
        let (_, col) = term.size();
        let upper_frame = format!(
            "\u{250f}{}\u{2513}",
            "\u{2501}".to_string().repeat(col as usize - 2)
        );
        term.write_line(&upper_frame)?;

        let make_line = |content: &Line, prompt: char| {
            format!(
                "\u{2503} {}{} \u{2503}",
                console::pad_str(
                    content.to_string().as_str(),
                    col as usize - 5,
                    Alignment::Left,
                    None
                ),
                prompt
            )
        };
        let first_line = make_line(&self.lines_inner.first, ' ');
        let second_line = make_line(
            &self.lines_inner.second,
            if show_prompt { self.prompt() } else { ' ' },
        );

        term.write_line(&first_line)?;
        term.write_line(&second_line)?;

        let lower_frame = format!(
            "\u{2517}{}\u{251b}",
            "\u{2501}".to_string().repeat(col as usize - 2)
        );
        term.write_line(&lower_frame)?;

        if show_picture {
            for line in self.picture.lines() {
                term.write_line(&console::pad_str(
                    line.trim(),
                    col as usize,
                    Alignment::Center,
                    None,
                ))?;
            }
        }

        Ok(())
    }
}

pub fn print_messages(
    term: &Term,
    interval: Duration,
    skip: bool,
    message_blocks: Vec<Vec<String>>,
    picture: &str,
) -> Result<()> {
    term.clear_screen()?;
    term.hide_cursor()?;

    let mut show_prompt = false;
    let mut lines_wraped = None;
    for message_block in message_blocks.into_iter() {
        let mut lines = Lines::new(message_block, &picture);
        loop {
            let ts = lines.typing();

            lines.print(&term, show_prompt, true)?;
            thread::sleep(interval);
            show_prompt = !show_prompt;

            match (ts, skip) {
                (TS::Waiting | TS::Done, true) => thread::sleep(interval),
                (TS::Waiting | TS::Done, false) => {
                    let term_2 = term.clone();
                    let handle = thread::spawn(move || term_2.read_key());

                    while !handle.is_finished() {
                        lines.print(&term, show_prompt, true)?;
                        thread::sleep(interval);
                        show_prompt = !show_prompt;
                    }

                    handle.join().unwrap()?;
                }
                _ => {}
            }

            if let TS::Done = ts {
                break;
            }
        }

        lines_wraped = Some(lines);
    }

    if let Some(lines) = lines_wraped {
        term.clear_screen()?;
        lines.print(&term, false, false)?;
    }

    term.show_cursor()?;

    Ok(())
}
