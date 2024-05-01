use anyhow::Result;
use console::Alignment;
use console::Term;
use dialoguer::{Input, Select};
use regex::{Captures, Regex};
use std::fmt;
use std::thread;
use std::time::Duration;

const PROMPT: char = '\u{25bc}';
const INTERVAL: Duration = Duration::from_millis(100);

// AA converter: https://tool-taro.com/image_to_ascii/
// Chrome image: https://ja.m.wikipedia.org/wiki/%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB:Google_Chrome_icon_%28February_2022%29.svg
// Ferris image: https://rustacean.net/
// Ferris mini aa: https://github.com/rust-lang/ferris-says/blob/main/src/lib.rs
// Gopher image: https://go.dev/doc/gopher/frontpage.png
// Python image: https://ja.m.wikibooks.org/wiki/%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB:Python-logo-notext.svg

fn scene_1() -> (Vec<Vec<String>>, String) {
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

fn scene_2() -> (Vec<Vec<String>>, String) {
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

fn scene_3() -> (Vec<Vec<String>>, String) {
    let messages = vec![vec![
        "では　はじめに　きみの　なまえを",
        "おしえて　もらおう！",
    ]]
    .into_iter()
    .map(|v| v.into_iter().map(|s| s.to_string()).collect())
    .collect();

    (messages, encolor(ferris_aa()))
}

fn scene_4(user_name: &str) -> (Vec<Vec<String>>, String) {
    let messages = vec![vec![
        "ふむ・・・",
        format!("{}　と　いうんだな！", user_name).as_str(),
    ]]
    .into_iter()
    .map(|v| v.into_iter().map(|s| s.to_string()).collect())
    .collect();

    (messages, encolor(ferris_aa()))
}

fn scene_5() -> (Vec<Vec<String>>, String) {
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

fn scene_6(rival_name: &str) -> (Vec<Vec<String>>, String) {
    let messages = vec![vec![
        "そうだ　そうだ！おもいだしたぞ",
        format!("{}　という　なまえだ", rival_name).as_str(),
    ]]
    .into_iter()
    .map(|v| v.into_iter().map(|s| s.to_string()).collect())
    .collect();

    (messages, encolor(gopher_aa()))
}

fn scene_7(user_name: &str) -> (Vec<Vec<String>>, String) {
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

/*
             MMHHHZZZZZZZuZuuZuXHHHMM
          MMHZZZZZZZZZZuZZuZuuuuuuuuuHHMM
       MMHZZZZZZZZZZZuZZuZZZuZuuZuuuuuuuWMM
     MMHyZZyyZZZZZZZZZZuZuuZuZuZuuuuuuuuuuWMM
   MMHyZZyZZZZyZZZZZZZZZZuZuZuuuZuuuuuuuuuuXHMM
 MMHyyyZyZZZyZZZZZZZZuZuZZuZuZuuZuuuuuuuuuuuuXWMM
MM9UyyZyZyZZZZZZZZX0V7<!!!!?<?<<<<<<<<<<<<<<<<?WMM
M8zzXZyZZyZyZZZZ0C!`  ......   `~::::::::::::::?WM
NzzzzXZyZZZZyZ0v` .(uwZZZZZXA&-   ~:::::::::::::JM
zzzzzzuZyyZZyV! .JwZZZZZZZyZZZXw-  _~::::::;:;::;;
zzzzzuuuZZZyU: .jZZZZZZZZZZZZZZZX+  _:::::::::::::
zzuzzzzzuZZZ>` (ZZZyZZyZZyZZyZZZZ0_  (::::::::::::
zzzuzuzuuuZZ; -dZZZZZZZZZZZZZZZZZX>  (::::::::::::
zuzzuuzuuzuZ: .zZZyZZyZZyZZyZZyZZX>  _::::::::::::
uuuzuuzuuuuuo. (XZZZZZZZZZZZZZZZZV~ ._::::::::::::
uzuuuzuuuuuuu:  (XZZyZZyZZyZZyZXC!  _:::::::::::::
kuzuuuuuuuuuuui. (CXZZZZZZZZZXVC``._:::~::::::::++
NuuuuuuuuuuuuuuO-.  ?7TTVVTTC!  .(<::::::~::::::JM
MNXuuuuuuuuuuuuuuAx-..      ..(uZ<::~:~:::~:~::jNM
MMNXuuuuuuuuZuZZZuZZXw&&&&&uwZZV<::~::~~:::~::jNMM
 MMNkuuuuZuZZuZZZZZZZZZZZZZZZXZ<:~::~:::~:::(jNMM
   MMNkZZuZZZZZuZZZZZZZZZZZZ0C<:~:~:~:~:~~:jdMM
     MMHkZuZuZZZuZZZZZZZZZZ0C<:~:~:~:~:::jdMM
       MMHkZZZZZZZZZZZZZZZ0C<~~:~::~::~(dMM
          MMHQkZZZZZZZZZy0I:~:~:~~((JdNMM
            MMNNNHZZyZZy0I~~:~:(qNNMM
*/

fn google_aa() -> String {
    #[rustfmt::skip]
    let aa = r#".             MMHHHr[ZZZZZZZuZuuZu]XHHHMM             .
.          MMHr[ZZZZZZZZZZuZZuZuuuuuuuuu]HHMM         .
.       MMHr[ZZZZZZZZZZZuZZuZZZuZuuZuuuuuuu]WMM       .
.     MMHr[yZZyyZZZZZZZZZZuZuuZuZuZuuuuuuuuuu]WMM     .
.   MMHr[yZZyZZZZyZZZZZZZZZZuZuZuuuZuuuuuuuuuu]XHMM   .
. MMHr[yyyZyZZZyZZZZZZZZuZuZZuZuZuuZuuuuuuuuuuuu]XWMM .
.MMg[9]r[UyyZyZyZZZZZZZZX0V7<!!!!?<?<<<<<<<<<<<<<<<<?]WMM.
.Mg[8zz]r[XZyZZyZyZZZZ0C!`]  b[......]   Y[`~::::::::::::::?]WM.
.Ng[zzzz]r[XZyZZZZyZ0v`] b[.(uwZZZZZXA&-]   Y[~:::::::::::::]JM.
.g[zzzzzzu]r[ZyyZZyV!] b[.JwZZZZZZZyZZZXw-]  Y[_~::::::;:;::;;].
.g[zzzzzuuu]r[ZZZyU:] b[.jZZZZZZZZZZZZZZZX+]  Y[_:::::::::::::].
.g[zzuzzzzzu]r[ZZZ>`] b[(ZZZyZZyZZyZZyZZZZ0_]  Y[(::::::::::::].
.g[zzzuzuzuuu]r[ZZ;] b[-dZZZZZZZZZZZZZZZZZX>]  Y[(::::::::::::].
.g[zuzzuuzuuzu]r[Z:] b[.zZZyZZyZZyZZyZZyZZX>]  Y[_::::::::::::].
.g[uuuzuuzuuuuuo]r[.] b[(XZZZZZZZZZZZZZZZZV~] Y[._::::::::::::].
.g[uzuuuzuuuuuuu:]  b[(XZZyZZyZZyZZyZXC!]  Y[_:::::::::::::].
.g[kuzuuuuuuuuuuui.] b[(CXZZZZZZZZZXVC``]Y[._:::~::::::::++].
.Ng[uuuuuuuuuuuuuuO-.]  b[?7TTVVTTC!]  Y[.(<::::::~::::::]JM.
.MNXg[uuuuuuuuuuuuuuAx-..]      g[..(uZ]Y[<::~:~:::~:~::j]NM.
.MMNXg[uuuuuuuuZuZZZuZZXw&&&&&uwZZV]Y[<::~::~~:::~::j]NMM.
. MMNkg[uuuuZuZZuZZZZZZZZZZZZZZZXZ]Y[<:~::~:::~:::(j]NMM .
.   MMNkg[ZZuZZZZZuZZZZZZZZZZZZ0C]Y[<:~:~:~:~:~~:j]dMM   .
.     MMHkg[ZuZuZZZuZZZZZZZZZZ0C]Y[<:~:~:~:~:::j]dMM     .
.       MMHkg[ZZZZZZZZZZZZZZZ0C]Y[<~~:~::~::~(d]MM       .
.          MMHQkg[ZZZZZZZZZy0I]Y[:~:~:~~((Jd]NMM         .
.            MMNNNHg[ZZyZZy0]Y[I~~:~:(q]NNMM             ."#;
    aa.to_string()
}

/*



MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM9MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM#zzVMM#zzzUMMBzzXMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMNSuuWMMzzzzzXzzzzzXXzzuzWMBUzzMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMWMMM#zzzzzzuzuzzzuzuzzzzuzzzzzzzzzWMMMBMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMRzzzzXzzzuzzzuzzuzzuzuzzuzzuzzuzzzzzUzzzzMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMM@zzzzzzuzzuzzzzuzzzzzzuzzzuzzuzzuzuzzzzuXMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMNkzzzzzzuzuzzzuzzuzuzzzuzuzzzuzzzzuzzzzuzzuzzzzzuzzzMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMRzzzzzuzzzuzzzuzzzzuzzzuzuzzzuzzuzzuzzzzuzuzzuzzuzXMMMMMMMMMMMMMMM
MMMMMMMMMMMWWHBzuzuzzzuzzuzzzuzzuzzuzzzzuzzzuzzzuzuzuzzzzuzzuzzzXMBWHMMMMMMMMMMM
MMMMMMMMMNkzzzzuzzzuzzzuzzuzzzuzzzuzzuzzuzuzzuzzzzzuzzuzzzuzzzuzzzzzudMMMMMMMMMM
MMMMMMMMMMNwzzuzzuzzuzzzuzzuzzzuzzzzuzzuzzzuzzuzuzzzzuzuzzzuzzzuzuzzdMMMMMMMMMMM
MMMMMMMMHBUXzzzzuzzzzuzzzuzzuzzzuzzuzzzzzuzzzzzuzuzuzzzzuzzzuzzzuzzuUWHMMMMMMMMM
MMMMMMNXzzzzzuzzzuzuzzuzzzuzzuzz7TQgmXzuzzuzuw74NmyzuzzuzzuzzuzzzzuzzzzzdMMMMMMM
MMMMMMMNkuzzuzuzzzuzzzuzuzzzzuu)  ,MMMkzzzzzq\  (MMKzuzzzuzzzuzuzzzzuzXdMMMMMMMM
MMMMMMMM8zuzzzzuzzzuzzzuzzuzzzdR. .MMM#zuzzuMm..dMMNzzuzzzuzzzuzuzzuzzXMMMMMMMMM
MMMMMM8zzzzuzzuzzuzzuzzzzuzzuzdMMMMMMM#zzuzzMMMMMMMMzzzuzzzuzzzzzuzzuzzuuWMMMMMM
MMMM8zzzuzuzzuzzuzzzzuzuzzzuzzzVMMMMMMuzzzzzzMMMMMMUzuzzuzzzuzuzzzuzuzzzzzzXHMMM
MMNzzzzuXkbbWkXXzzuzzzuzzuzzzuzzzzzzzzzuzuzzuzzXXXzzuzzzuzuzzzzuzuXWHbWkzuzzXMMM
MMMmXzuzzMNkpWNNmzzuXXXzzzXQHMHHMMNmXzzzuzzXQH8UzuXWmXzuzXXkXXzzXdMMbWMHuzzXMMMM
MMMMNmuzzuMMNbWMMmXzzXWWkH8zzzzzzzuXMNXuzXdBzzzzzzzuzVNkWWUzzzuqMMMbWMMzzzqMMMMM
MMMMMMNmuzzWMNHWMMNmzzzXSzuzzuzzuzzzzXMNkMXXXXXXzzuzzuzUzzzuXgMMMMWNMMSuXdMMMMMM
MMMMMMMMNmzzUMMNWMMMNmXzzzzuzuWQNNNMNMMMMMMMMMMMMMUzzzuzXQQMMMMMMHMMM8zQMMMMMMMM
MMMMMMMMMMNmuXMMMNMMMMMNNyzzzzzXWMMMMMMMMMMMMMMMUzzuzzXdMMMMMMMMNMMM8zdMMMMMMMMM
MMMMMMMMMMMMNmXMMMMMMMMMMMmXuzzzzzzzXWMMMHHUUzzzzzzzXgMMMMMMMMMMMMM#XNMMMMMMMMMM
MMMMMMMMMMMMMMNkMMMMMMMMMMMMNmXzzzzXqMMMMMNmXXuXXQQMMMMMMMMMMMMMMMNWMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNNNMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM


*/

fn ferris_aa() -> String {
    #[rustfmt::skip]
    let aa = r#"
.                                                                         .
.                             r[zz]    r[zzz]    r[zz]                             .
.                       r[uu]   r[zzzzz] r[zzzzz]  r[zzuz]    r[zz]                      .
.                      r[zzzzzzuzuzzzuzuzzzzuzzzzzzzzz]                      .
.                 r[zzzz] r[zzzuzzzuzzuzzuzuzzuzzuzzuzzzzz] r[zzzz]                .
.                 r[zzzzzzuzzuzzzzuzzzzzzuzzzuzzuzzuzuzzzzu]                 .
.            r[zzzzzzuzuzzzuzzuzuzzzuzuzzzuzzzzuzzzzuzzuzzzzzuzzz]           .
.            r[zzzzzuzzzuzzzuzzzzuzzzuzuzzzuzzuzzuzzzzuzuzzuzzuz]            .
.            r[zuzuzzzuzzuzzzuzzuzzuzzzzuzzzuzzzuzuzuzzzzuzzuzzz]            .
.        r[zzzzuzzzuzzzuzzuzzzuzzzuzzuzzuzuzzuzzzzzuzzuzzzuzzzuzzzzzu]       .
.         r[zzuzzuzzuzzzuzzuzzzuzzzzuzzuzzzuzzuzuzzzzuzuzzzuzzzuzuzz]        .
.         r[zzzzuzzzzuzzzuzzuzzzuzzuzzzzzuzzzzzuzuzuzzzzuzzzuzzzuzzu]        .
.     r[zzzzzuzzzuzuzzuzzzuzzuzz]      r[zuzzuzu]      r[zuzzuzzuzzuzzzzuzzzzz]    .
.      r[uzzuzuzzzuzzzuzuzzzzuu]MMM     r[zzzzz] MMM    r[zuzzzuzzzuzuzzzzuz]      .
.      r[zuzzzzuzzzuzzzuzzuzzzz]MMMM    r[zuzzu] MMM    r[zzuzzzuzzzuzuzzuzz]      .
.    r[zzzzuzzuzzuzzuzzzzuzzuzz]        r[zzuzz]        r[zzzuzzzuzzzzzuzzuzzuu]   .
.  r[zzzuzuzzuzzuzzzzuzuzzzuzzzz]      r[uzzzzzz]      r[zzuzzuzzzuzuzzzuzuzzzzzz] .
.r[zzzzu]        r[zzuzzzuzzuzzzuzzzzzzzzzuzuzzuzzzzzzzuzzzuzuzzzzuzu]      r[zuzz].
.  r[zuzz]        r[zzu]   r[zzz]           r[zzzuzz]     r[zu]    r[zuz]     r[zz]        r[uzz] .
.   r[uzzu]         r[zz]      r[zzzzzzzu]    r[uz]   r[zzzzzzzuz]      r[zzzu]        r[zzz]  .
.     r[uzz]         r[zzz]  r[zuzzuzzuzzzz]           r[zzuzzuz] r[zzzu]           r[u]    .
.       r[zz]           r[zzzzuzu]                     r[zzzuz]              r[z]     .
.         r[u]             r[zzzzz]                  r[zzuzz]               r[z]      .
.                         r[uzzzzzzz]         r[zzzzzzz]                        .
.                            r[zzzz]           r[u]                             ."#;
    aa.to_string()
}

/*

    _~^~^~_
\) /  o o  \ (/
  '_   -   _'
  / '-----' \

*/

fn mini_ferris_aa() -> String {
    #[rustfmt::skip]
    let aa = r#"
    r[_~^~^~_]
r[\) /  o o  \ (/]
  r['_   -   _']
  r[/ '-----' \]    
"#;
    aa.to_string()
}

/*
               ..-xZY""""""""""Y0+..
          ..Z"!                ..  ."4,...JJ..
  .Y"77"HY^ .,77?7!..      ./^    .7.  Ta    .W.
 K   ..J^ .7        .i    ,`         1.  WMN.  b
.]  (M#   \....       t  .!.NN,       t   WF  .F
 W,  d`  .(MM#H       (  .,MM@d       c    N..Y
  ?5j%    t?WB^      .\   1.?^       .'    ,]
    d      1.       .^.gNaJ(+.     .?`      W
   .F        ?7<(?7!.?MMMM@:.  ??!          -|
   .]              J         1              .]
   ,\              .=</7J?7,(^               F
   -}                .` (  r                 @
   ,]                .i."i.^                 @
   .]                                        @
    N                                        @
    d.                                       @
    ({                                       @
 ..?4)                                       #_?!,
r.  -}                                       H. <.
(..?d~                                       d 77!
    d                                        d
    d                                        J~
    W                                        ({
    k                                        ,}
    H                                        -}
    H                                        J`
    W                                        K
    W                                       .F
    d.                                      d
    .h                                     .^
     .N.                                  .3
       Tx                               .V`
       .?^?=.                        .,!  1.
      r.!  ..SJ..               ...v"!7, .,(.
      1\ .?     _7""TTTVVTY""""^`       1.,?
*/

fn gopher_aa() -> String {
    #[rustfmt::skip]
    let aa = r#"

.               c[..-xZY""""""""""Y0+..]              .
.          c[..Z"!]                ..  c[."4,...JJ..]    .
.  c[.Y"77"HY^] .,77?7!..      ./^MMMM.7.  c[Ta    .W.]  .
. c[K   ..J^] .7MMMMMMMM.i    ,`MMMMMMMMM1.  c[WMN.  b]  .
.c[.|  (M#]   \````MMMMMMMt  .!`  `MMMMMMMt   c[WF  .F]  .
. c[W,  d`]  .     MMMMMMM(  .,    MMMMMMMc    c[N..Y]   .
.  c[?5j%]    t    MMMMMM.\   1   MMMMMMM.'    c[,|]     .
.    c[d]      1`MMMMMMM`^.gNaJ(+.MMMMM.?`      c[W]     .
.   c[.F]        ?7<(?7!.    M@:.  ??!          c[-|]    .
.   c[.|]              J"""""""""1              c[.|]    .
.   c[,\]              .=</7J?7,(^               c[F]    .
.   c[-}]                .` (  r                 c[@]    .
.   c[,|]                .i."i.^                 c[@]    .
.   c[.|                                        @]    .
.   c[ N                                        @]    .
.   c[ d.                                       @]    .
.   c[ ({                                       @]    .
. c[..?4)                                       #_?!,].
.c[r.  -}                                       H. <.].
.c[(..?d~                                       d 77!].
.    c[d                                        d]    ."#;
    /*
        b[d                                        J~]
        b[W                                        ({]
        b[k                                        ,}]
        b[H                                        -}]
        b[H                                        J`]
        b[W                                        K]
        b[W                                       .F]
        b[d.                                      d]
        b[.h                                     .^]
         b[.N.                                  .3]
           b[Tx                               .V`]
           b[.?^?=.                        .,!  1.]
          b[r.!  ..SJ..               ...v"!7, .,(.]
          b[1\ .?     _7""TTTVVTY""""^`       1.,?]
    */
    aa.to_string()
}

/*
.                 ...wwXffffppppppbbkkkJ.                    .
.               .JVVVVfffppppfppppbbkkkkkH,                  .
.               dVVVY???pfppppppbbkkkkkqkqqe                 .
.              .VVVR    (pfppppbbkkkkqqqqqqH{                .
.              .VVVfo,..pppppbbkkkkqkqqqqqmm}                .
.              .fffpffpppppbbbkkkkqkqqqqqqmg}                .
.              .ffpfpppppbbbkkkkqqqqqqqqmmgg}                .
.      .......................qkqqqqqqqmmggg} .......        .
.   .JyVVVfffffppppppbbbkkkkkqqqqqqqmmmggggg} .~..~.~._.     .
.  JyVVVffffpppfpppppbbkkkkqqkqqqqqmmmgggggg} ..~..~.~~~_    .
. .VVVVVfffppfpppppbbkkkkkqqqqqqqqmggggggggg} .~.~~.~.~~~_   .
..yVVVVffppfpppppbbbkkkqqkqqqqqqmmgggggggggH} .~~.~~~~.~~~.  .
.JVVVVffppppppppbbkkkkkqqqqqqqmmgggggggggggY .~~.~.~~~~~~~_  .
.VVVffppfpppppbbkkkkqqqqqqqqqmmggggggggggH= ~~.~~~~~~~~~~~~  .
.ffffpppppppbbbkkkkqWWWWWWWHHHHHHHHHHH""!--~~.~~~~~~~~~~~~~. .
.fppppfppppbbkkkk9^`  .................~~~~~~~~~~~~~~~~~~~:_ .
.fpfpppppbbkkkkH= ................~.~.~.~~~.~~~~~~~~~~~~::_  .
.Wpppppbbkkkkkk` ...........~.~~.~~.~~~~~~~~~~~~~~~~~:~:~:_  .
..ppppbbkkkkqq] ..~.~.~.~~.~~.~~~~~~~~~~~~~~~~~~~~~:~::~::`  .
. 4ppbkkkkkqqq] .....~.~.~~.~~..~~.~~~~~~~~~~~~:~::~:~::~~   .
. .4bkkkkqqqqq] .~.~.~.~..~~.~~~~~~~~~~~~~~~~:~:~:~::~::~    .
.   TWkkqqqqqq] .~~.~.~~~~~~~~~~~~~~~~~~~~~::~:~::~:~:~`     .
.     ???T"T""\ ~..~~~.~~~.~~~_``______~~~~~~~~~~~~```       .
.               ~~.~~~~~~~~~~~~~~~~~~~~:~:~:.                .
.               .~~~.~~~~~~~~~~~~~~~:~::::::`                .
.               ~~~~~~~~~~~~~~~~:~::`  _:~~:`                .
.               ~~~~~~~~~~~~~::~::~_    _:::.                .
.                ~~~~~~~~~:~:~~::~::. ..:~:`                 .
.                 _~~~~:~:~::::~:~:~:::::~`                  .
.                   `~~~:::~:~::~:::~~~`                     .
.                          ``````                            .
*/

fn python_aa() -> String {
    #[rustfmt::skip]
    let aa = r#"
.                 b[...wwXffffppppppbbkkkJ.]                    .
.               b[.JVVVVfffppppfppppbbkkkkkH,]                  .
.               b[dVVVY???pfppppppbbkkkkkqkqqe]                 .
.              b[.VVVR    (pfppppbbkkkkqqqqqqH{]                .
.              b[.VVVfo,..pppppbbkkkkqkqqqqqmm}]                .
.              b[.fffpffpppppbbbkkkkqkqqqqqqmg}]                .
.              b[.ffpfpppppbbbkkkkqqqqqqqqmmgg}]                .
.      b[.......................qkqqqqqqqmmggg}] Y[.......]        .
.   b[.JyVVVfffffppppppbbbkkkkkqqqqqqqmmmggggg}] Y[.~..~.~._.]     .
.  b[JyVVVffffpppfpppppbbkkkkqqkqqqqqmmmgggggg}] Y[..~..~.~~~_]    .
. b[.VVVVVfffppfpppppbbkkkkkqqqqqqqqmggggggggg}] Y[.~.~~.~.~~~_]   .
.b[.yVVVVffppfpppppbbbkkkqqkqqqqqqmmgggggggggH}] Y[.~~.~~~~.~~~.]  .
.b[JVVVVffppppppppbbkkkkkqqqqqqqmmgggggggggggY] Y[.~~.~.~~~~~~~_]  .
.b[VVVffppfpppppbbkkkkqqqqqqqqqmmggggggggggH=] Y[~~.~~~~~~~~~~~~]  .
.b[ffffpppppppbbbkkkkqWWWWWWWHHHHHHHHHHH""!]Y[--~~.~~~~~~~~~~~~~.] .
.b[fppppfppppbbkkkk9^`]  Y[.................~~~~~~~~~~~~~~~~~~~:_] .
.b[fpfpppppbbkkkkH=] Y[................~.~.~.~~~.~~~~~~~~~~~~::_]  .
.b[Wpppppbbkkkkkk`] Y[...........~.~~.~~.~~~~~~~~~~~~~~~~~:~:~:_]  .
.b[.ppppbbkkkkqq|] Y[..~.~.~.~~.~~.~~~~~~~~~~~~~~~~~~~~~:~::~::`]  .
. b[4ppbkkkkkqqq|] Y[.....~.~.~~.~~..~~.~~~~~~~~~~~~:~::~:~::~~]   .
. b[.4bkkkkqqqqq|] Y[.~.~.~.~..~~.~~~~~~~~~~~~~~~~:~:~:~::~::~]    .
.   b[TWkkqqqqqq|] Y[.~~.~.~~~~~~~~~~~~~~~~~~~~~::~:~::~:~:~`]     .
.     b[???T"T""\] Y[~..~~~.~~~.~~~_``______~~~~~~~~~~~~```]       .
.               Y[~~.~~~~~~~~~~~~~~~~~~~~:~:~:.]                .
.               Y[.~~~.~~~~~~~~~~~~~~~:~::::::`]                .
.               Y[~~~~~~~~~~~~~~~~:~::`  _:~~:`]                .
.               Y[~~~~~~~~~~~~~::~::~_    _:::.]                .
.                Y[~~~~~~~~~:~:~~::~::. ..:~:`]                 .
.                 Y[_~~~~:~:~::::~:~:~:::::~`]                  .
.                   Y[`~~~:::~:~::~:::~~~`]                     .
.                          Y[``````]                            ."#;
    aa.to_string()
}

fn encolor(s: String) -> String {
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
    let s = color_regex_block!(s, r"Y\[(.*?)\]", yellow);
    let s = color_regex_block!(s, r"c\[(.*?)\]", cyan);

    s.to_string()
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

fn print_messages(term: &Term, message_blocks: Vec<Vec<String>>, picture: &str) -> Result<()> {
    term.clear_screen()?;
    term.hide_cursor()?;

    let mut show_prompt = false;
    let mut lines_wraped = None;
    for message_block in message_blocks.into_iter() {
        let mut lines = Lines::new(message_block, &picture);
        while let TS::Typing = lines.typing() {
            lines.print(&term, show_prompt, true)?;
            thread::sleep(INTERVAL);
            show_prompt = !show_prompt;
        }

        let term_2 = term.clone();

        let handle = thread::spawn(move || term_2.read_key());

        while !handle.is_finished() {
            lines.print(&term, show_prompt, true)?;
            thread::sleep(INTERVAL);
            show_prompt = !show_prompt;
        }

        handle.join().unwrap()?;
        lines_wraped = Some(lines);
    }

    if let Some(lines) = lines_wraped {
        term.clear_screen()?;
        lines.print(&term, false, false)?;
    }

    term.show_cursor()?;

    Ok(())
}

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
