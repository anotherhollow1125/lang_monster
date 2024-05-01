use anyhow::Result;
use console::Alignment;
use console::Term;
use std::fmt;
use std::thread;
use std::time::Duration;

const PROMPT: char = '\u{25bc}';

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
    Done,
}

struct Lines<'a> {
    messages: Vec<Vec<char>>,
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
}

use TypingStatus as TS;

impl<'a> Lines<'a> {
    fn new(messages: Vec<String>, picture: &'a str) -> Self {
        let mut messages: Vec<Vec<char>> = messages
            .into_iter()
            .map(|line| line.chars().rev().collect())
            .collect();
        messages.reverse();
        Self {
            messages,
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
            TS::Done => PROMPT,
        }
    }

    fn typing(&mut self) -> TS {
        let Lines {
            ref mut messages,
            ref mut lines_inner,
            ..
        } = self;

        let Some(message) = messages.last_mut() else {
            self.status = TS::Done;
            return TS::Done;
        };

        let target_line = lines_inner.target_line();

        let c = message.pop();
        if let Some(c) = c {
            target_line.typing(c);
        }

        if message.is_empty() {
            messages.pop();
            target_line.into_done();
        }

        TS::Typing
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
        while let TS::Typing = lines.typing() {
            lines.print(&term, show_prompt, true)?;
            thread::sleep(interval);
            show_prompt = !show_prompt;
        }

        let term_2 = term.clone();

        if skip {
            thread::sleep(interval);
        } else {
            let handle = thread::spawn(move || term_2.read_key());

            while !handle.is_finished() {
                lines.print(&term, show_prompt, true)?;
                thread::sleep(interval);
                show_prompt = !show_prompt;
            }

            handle.join().unwrap()?;
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
