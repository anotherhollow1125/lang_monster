use regex::{Captures, Regex};

pub mod ascii_arts;
use ascii_arts::{ferris_aa, google_aa, gopher_aa, python_aa};

pub fn encolor(s: String) -> String {
    macro_rules! color_regex_block {
        ( $s:expr, $r:expr, $color:ident ) => {{
            Regex::new($r).unwrap().replace_all(&$s, |cap: &Captures| {
                format!("{}", console::style(&cap[1]).$color())
            })
        }};
    }

    let s = color_regex_block!(s, r"r\[(.*?)\]", red);
    let s = color_regex_block!(s, r"g\[(.*?)\]", green);
    let s = color_regex_block!(s, r"b\[(.*?)\]", blue);
    let s = color_regex_block!(s, r"y\[(.*?)\]", yellow);
    let s = color_regex_block!(s, r"c\[(.*?)\]", cyan);

    s.to_string()
}

pub fn scene_1() -> (Vec<Vec<String>>, String) {
    let messages = vec![
        vec![
            "はじめまして！",
            "ランゲージ　モンスターの　せかいへ",
            "ようこそ！",
        ],
        vec![
            "わたしの　なまえは　グーグル",
            "みんなからは　ラグモン　はかせと",
            "したわれて　おるよ",
        ],
    ]
    .into_iter()
    .map(|v| v.into_iter().map(|s| s.to_string()).collect())
    .collect();

    (messages, encolor(google_aa()))
}

pub fn scene_2() -> (Vec<Vec<String>>, String) {
    let messages = vec![
        vec!["この　せかいには", "ランゲージ　モンスターと　よばれる"],
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
    ]
    .into_iter()
    .map(|v| v.into_iter().map(|s| s.to_string()).collect())
    .collect();

    (messages, encolor(python_aa()))
}

pub fn scene_3() -> (Vec<Vec<String>>, String) {
    let messages = vec![vec![
        "では　はじめに　きみの　なまえを",
        "おしえて　もらおう！",
    ]]
    .into_iter()
    .map(|v| v.into_iter().map(|s| s.to_string()).collect())
    .collect();

    (messages, encolor(ferris_aa()))
}

pub fn scene_4(user_name: &str) -> (Vec<Vec<String>>, String) {
    let messages = vec![vec![
        "ふむ・・・",
        format!("{}　と　いうんだな！", user_name).as_str(),
    ]]
    .into_iter()
    .map(|v| v.into_iter().map(|s| s.to_string()).collect())
    .collect();

    (messages, encolor(ferris_aa()))
}

pub fn scene_5() -> (Vec<Vec<String>>, String) {
    let messages = vec![
        vec![
            "こいつは　わたしの　まご",
            "きみの　おさななじみであり",
            "ライバル　である",
        ],
        vec!["・・・えーと？", "なまえは　なんて　いったかな？"],
    ]
    .into_iter()
    .map(|v| v.into_iter().map(|s| s.to_string()).collect())
    .collect();

    (messages, encolor(gopher_aa()))
}

pub fn scene_6(rival_name: &str) -> (Vec<Vec<String>>, String) {
    let messages = vec![vec![
        "そうだ　そうだ！おもいだしたぞ",
        format!("{}　という　なまえだ", rival_name).as_str(),
    ]]
    .into_iter()
    .map(|v| v.into_iter().map(|s| s.to_string()).collect())
    .collect();

    (messages, encolor(gopher_aa()))
}

pub fn scene_7(user_name: &str) -> (Vec<Vec<String>>, String) {
    let messages = vec![
        vec![format!("{}！", user_name).as_str()],
        vec!["いよいよ　これから", "きみの　ものがたりの　はじまりだ！"],
        vec![
            "ゆめと　ぼうけんと！",
            "ランゲージ　モンスターの　せかいへ！",
            "レッツゴー！",
        ],
    ]
    .into_iter()
    .map(|v| v.into_iter().map(|s| s.to_string()).collect())
    .collect();

    (messages, encolor(ferris_aa()))
}
