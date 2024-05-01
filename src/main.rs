use anyhow::Result;
use console::Alignment;
use console::Term;
use dialoguer::{Input, Select};

mod scenes;
use scenes::ascii_arts::mini_ferris_aa;
use scenes::{encolor, scene_1, scene_2, scene_3, scene_4, scene_5, scene_6, scene_7};

mod message_box;
use message_box::print_messages;

fn main() -> Result<()> {
    let term = Term::stdout();

    let term1 = term.clone();
    ctrlc::set_handler(move || {
        term1.show_cursor().unwrap();
        std::process::exit(0);
    })?;

    let (message_blocks, picture) = scene_1();
    print_messages(&term, message_blocks, &picture)?;

    let (message_blocks, picture) = scene_2();
    print_messages(&term, message_blocks, &picture)?;

    let (message_blocks, picture) = scene_3();
    print_messages(&term, message_blocks, &picture)?;

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

    let (message_blocks, picture) = scene_4(&user_name);
    print_messages(&term, message_blocks, &picture)?;

    let (message_blocks, picture) = scene_5();
    print_messages(&term, message_blocks, &picture)?;

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

    let (message_blocks, picture) = scene_6(&rival_name);
    print_messages(&term, message_blocks, &picture)?;

    let (message_blocks, picture) = scene_7(&user_name);
    print_messages(&term, message_blocks, &picture)?;

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
