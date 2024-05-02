use anyhow::Result;
use clap::{Parser, ValueEnum};
use console::Alignment;
use console::Term;
use dialoguer::{Input, Select};
use std::time::Duration;

mod scenes;
use scenes::ascii_arts::mini_ferris_aa;
use scenes::{encolor, scene_1, scene_2, scene_3, scene_4, scene_5, scene_6, scene_7};

mod message_box;
use message_box::print_messages;

mod progress;
use progress::progress;

#[derive(Clone, Copy, Debug, ValueEnum)]
enum TextContinue {
    Auto,
    Manual,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Config {
    /// 文字送り
    #[arg(short, long, value_enum, default_value_t = TextContinue::Manual)]
    text_continue: TextContinue,

    /// はなしのはやさ (milli seconds)
    #[arg(short, long, default_value_t = 100)]
    speed: u64,

    /// 主人公の名前
    #[arg(short, long)]
    user_name: Option<String>,

    /// ライバルの名前
    #[arg(short, long)]
    rival_name: Option<String>,

    /// アセットダウンロード
    #[arg(short, long, default_value_t = false)]
    download_skip: bool,
}

fn ask_user_name() -> Result<String> {
    let user_name_cands = vec!["じぶんできめる", "ラスト", "フェリス", "スイフト"];

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

    Ok(user_name)
}

fn ask_rival_name() -> Result<String> {
    let rival_name_cands = vec!["じぶんできめる", "ゴー", "ゴーファー", "シープラプラ"];

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

    Ok(rival_name)
}

fn main() -> Result<()> {
    let Config {
        text_continue,
        speed,
        user_name: user_name_w,
        rival_name: rival_name_w,
        download_skip,
    } = Config::parse();

    let interval = Duration::from_millis(speed);
    let skip = match text_continue {
        TextContinue::Auto => true,
        TextContinue::Manual => false,
    };

    let term = Term::stdout();

    let term1 = term.clone();
    ctrlc::set_handler(move || {
        term1.show_cursor().unwrap();
        std::process::exit(0);
    })?;

    if !download_skip {
        progress(
            100,
            interval,
            "アセットダウンロード...".to_string(),
            "アセットダウンロード完了".to_string(),
        );
        progress(
            300,
            interval,
            "ワールド生成中...".to_string(),
            "ワールド生成完了".to_string(),
        );
    }

    let (message_blocks, picture) = scene_1();
    print_messages(&term, interval, skip, message_blocks, &picture)?;

    let (message_blocks, picture) = scene_2();
    print_messages(&term, interval, skip, message_blocks, &picture)?;

    let (message_blocks, picture) = scene_3();
    print_messages(&term, interval, skip, message_blocks, &picture)?;

    let user_name = match user_name_w {
        Some(name) => name,
        None => ask_user_name()?,
    };

    let (message_blocks, picture) = scene_4(&user_name);
    print_messages(&term, interval, skip, message_blocks, &picture)?;

    let (message_blocks, picture) = scene_5();
    print_messages(&term, interval, skip, message_blocks, &picture)?;

    let rival_name = match rival_name_w {
        Some(name) => name,
        None => ask_rival_name()?,
    };

    let (message_blocks, picture) = scene_6(&rival_name);
    print_messages(&term, interval, skip, message_blocks, &picture)?;

    let (message_blocks, picture) = scene_7(&user_name);
    print_messages(&term, interval, skip, message_blocks, &picture)?;

    let (_, col) = term.size();
    let mini_ferris = encolor(mini_ferris_aa());
    for line in mini_ferris.lines() {
        term.write_line(&console::pad_str(
            line.trim(),
            col as usize,
            Alignment::Center,
            None,
        ))?;
    }

    Ok(())
}
