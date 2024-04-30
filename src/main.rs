use anyhow::Result;
use console::Alignment;
use console::Term;
use dialoguer::{Input, Select};
use std::fmt;
use std::thread;
use std::time::Duration;

const PROMPT: char = '\u{25bc}';
const INTERVAL: Duration = Duration::from_millis(100);

fn message_1() -> Vec<Vec<String>> {
    vec![
        vec![
            "はじめまして！",
            "ラング　モンスターの　せかいへ",
            "ようこそ！",
        ],
        vec![
            "わたしの　なまえは　フェリス",
            "みんなからは　ラグモン　はかせと",
            "したわれて　おるよ",
        ],
        vec!["この　せかいには", "ラング　モンスターと　よばれる"],
        vec!["いきもの　たちが", "いたるところに　すんでいる！"],
        vec![
            "その　ラグモン　という　いきものを",
            "ひとは　ペットに　したり",
            "しょうぶに　つかったり・・・",
        ],
        vec!["そして・・・"],
        vec![
            "わたしは　この　ラグモンの",
            "けんきゅうを　してる　というわけだ",
        ],
        vec!["では　はじめに　きみの　なまえを", "おしえて　もらおう！"],
    ]
    .into_iter()
    .map(|v| v.into_iter().map(|s| s.to_string()).collect())
    .collect()
}

fn message_2(user_name: &str) -> Vec<Vec<String>> {
    vec![
        vec![
            "ふむ・・・",
            format!("{}　と　いうんだな！", user_name).as_str(),
        ],
        vec![
            "こいつは　わたしの　まご",
            "きみの　おさななじみであり",
            "ライバル　である",
        ],
        vec!["・・・えーと？", "なまえは　なんて　いったかな？"],
    ]
    .into_iter()
    .map(|v| v.into_iter().map(|s| s.to_string()).collect())
    .collect()
}

fn message_3(user_name: &str, rival_name: &str) -> Vec<Vec<String>> {
    vec![
        vec![
            "そうだ　そうだ！おもいだしたぞ",
            format!("{}　という　なまえだ", rival_name).as_str(),
        ],
        vec![format!("{}！", user_name).as_str()],
        vec!["いよいよ　これから", "きみの　ものがたりの　はじまりだ！"],
        vec![
            "ゆめと　ぼうけんと！",
            "ラング　モンスターの　せかいへ！",
            "レッツゴー！",
        ],
    ]
    .into_iter()
    .map(|v| v.into_iter().map(|s| s.to_string()).collect())
    .collect()
}

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

struct Lines {
    messages: Vec<Vec<char>>,
    lines_inner: LinesInner,
}

struct LinesInner {
    first: Line,
    second: Line,
    status: TypingStatus,
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

    fn prompt(&self) -> char {
        match self.status {
            TS::Typing => ' ',
            TS::Done => PROMPT,
        }
    }
}

use TypingStatus as TS;

impl Lines {
    fn new(messages: Vec<String>) -> Self {
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
                status: TS::Typing,
            },
        }
    }

    fn prompt(&self) -> char {
        self.lines_inner.prompt()
    }

    fn typing(&mut self) -> TS {
        let Lines {
            ref mut messages,
            ref mut lines_inner,
        } = self;

        let Some(message) = messages.last_mut() else {
            self.lines_inner.status = TS::Done;
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

    fn print(&self, term: &Term, show_prompt: bool) -> Result<()> {
        term.clear_screen()?;
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

        Ok(())
    }
}

fn print_messages(message_blocks: Vec<Vec<String>>) -> Result<()> {
    let term = Term::stdout();
    let mut show_prompt = false;
    let mut lines_wraped = None;
    for message_block in message_blocks.into_iter() {
        let mut lines = Lines::new(message_block);
        while let TS::Typing = lines.typing() {
            lines.print(&term, show_prompt)?;
            thread::sleep(INTERVAL);
            show_prompt = !show_prompt;
        }

        let term_2 = term.clone();

        let handle = thread::spawn(move || term_2.read_key());

        while !handle.is_finished() {
            lines.print(&term, show_prompt)?;
            thread::sleep(INTERVAL);
            show_prompt = !show_prompt;
        }

        handle.join().unwrap()?;
        lines_wraped = Some(lines);
    }

    if let Some(lines) = lines_wraped {
        lines.print(&term, false)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let message_blocks_1 = message_1();
    print_messages(message_blocks_1)?;

    let user_name_cands = vec!["じぶんできめる", "ラスト", "パイソン", "スイフト"];

    let selection = Select::new()
        .with_prompt("なまえこうほ")
        .items(&user_name_cands)
        .default(0)
        .interact()?;

    let user_name = match selection {
        0 => Input::new()
            .with_prompt("なまえをにゅうりょく")
            .interact_text()?,
        i => user_name_cands[i].to_string(),
    };

    let message_blocks_2 = message_2(&user_name);
    print_messages(message_blocks_2)?;

    let rival_name_cands = vec!["じぶんできめる", "ゴー", "ルビー", "シープラプラ"];

    let selection = Select::new()
        .with_prompt("なまえこうほ")
        .items(&rival_name_cands)
        .default(0)
        .interact()?;

    let rival_name = match selection {
        0 => Input::new()
            .with_prompt("なまえをにゅうりょく")
            .interact_text()?,
        i => rival_name_cands[i].to_string(),
    };

    let message_blocks_3 = message_3(&user_name, &rival_name);
    print_messages(message_blocks_3)?;

    Ok(())
}
