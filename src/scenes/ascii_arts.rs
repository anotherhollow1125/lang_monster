// AA converter: https://tool-taro.com/image_to_ascii/
// -> 解像度の関係で caca-utils の img2txt に変更
// Chrome image: https://ja.m.wikipedia.org/wiki/%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB:Google_Chrome_icon_%28February_2022%29.svg
// Ferris image: https://rustacean.net/
// Ferris mini aa: https://github.com/rust-lang/ferris-says/blob/main/src/lib.rs
// Gopher image: https://go.dev/doc/gopher/frontpage.png
// Gopher image 2: https://github.com/egonelbre/gophers/blob/master/icon/emoji/gopher-neutral.png
// Python image: https://ja.m.wikibooks.org/wiki/%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB:Python-logo-notext.svg

/*
|            RRRRRRRR            |
|        RRRRRRRRRRRRRRRR        |
|      RRRRRRRRRRRRRRRRRRRR      |
|    RRRRRRRRRRRRRRRRRRRRRRRR    |
|   RRRRRRRRRRRRRRRRRRRRRRRRRR   |
|  RRRRRRRRR         YYYYYYYYYY  |
| GGGRRRRR     BBBBB   YYYYYYYYY |
| GGGGRRRR   BBBBBBBBB  YYYYYYYY |
|GGGGGGRR  BBBBBBBBBBBB  YYYYYYYY|
|GGGGGGGR  BBBBBBBBBBBB  YYYYYYYY|
|GGGGGGGR  BBBBBBBBBBBB  YYYYYYYY|
|GGGGGGGG   BBBBBBBBBB   YYYYYYYY|
| GGGGGGGG   BBBBBBBB   YYYYYYYY |
| GGGGGGGGG            YYYYYYYYY |
|  GGGGGGGGGGG     GGYYYYYYYYYY  |
|   GGGGGGGGGGGGGGGGGYYYYYYYYY   |
|    GGGGGGGGGGGGGGGYYYYYYYYY    |
|       GGGGGGGGGGYYYYYYYY       |
|          GGGGGGYYYYYY          |
*/

pub fn google_aa() -> String {
    #[rustfmt::skip]
    let aa = r#"|            r[RRRRRRRR]            |
|        r[RRRRRRRRRRRRRRRR]        |
|      r[RRRRRRRRRRRRRRRRRRRR]      |
|    r[RRRRRRRRRRRRRRRRRRRRRRRR]    |
|   r[RRRRRRRRRRRRRRRRRRRRRRRRRR]   |
|  r[RRRRRRRRR]         y[YYYYYYYYYY]  |
| g[GGG]r[RRRRR]     b[BBBBB]   y[YYYYYYYYY] |
| g[GGGG]r[RRRR]   b[BBBBBBBBB]  y[YYYYYYYY] |
|g[GGGGGG]r[RR]  b[BBBBBBBBBBBB]  y[YYYYYYYY]|
|g[GGGGGGG]r[R]  b[BBBBBBBBBBBB]  y[YYYYYYYY]|
|g[GGGGGGG]r[R]  b[BBBBBBBBBBBB]  y[YYYYYYYY]|
|g[GGGGGGGG]   b[BBBBBBBBBB]   y[YYYYYYYY]|
| g[GGGGGGGG]   b[BBBBBBBB]   y[YYYYYYYY] |
| g[GGGGGGGGG]            y[YYYYYYYYY] |
|  g[GGGGGGGGGGG     GG]y[YYYYYYYYYY]  |
|   g[GGGGGGGGGGGGGGGGG]y[YYYYYYYYY]   |
|    g[GGGGGGGGGGGGGGG]y[YYYYYYYYY]    |
|       g[GGGGGGGGGG]y[YYYYYYYY]       |
|          g[GGGGGG]y[YYYYYY]          |"#;
    aa.to_string()
}

/*
|             R RR R             |
|        RRR RR RR RRRR R        |
|      RR RRRRRRRRRRRRRRRRR      |
|    RRRRRRRRRRRRRRRRRRRRRRR     |
|    RRRRRRRRRRRRRRRRRRRRRRR     |
|   RRRRRRRRRRW RRRW RRRRRRRRR   |
|  RRRRRRRRRRR  RRR  RRRRRRRRRR  |
| R RRRRR  RRRRRRRRRRRRR  RRRRRR |
|  RR R RRRRR RRRRRRR  RRR   RR  |
|    R     RRR      RRR          |
|            RR    R             |
*/

pub fn ferris_aa() -> String {
    #[rustfmt::skip]
    let aa = r#"|                                |
|             r[R RR R]             |
|        r[RRR RR RR RRRR R]        |
|      r[RR RRRRRRRRRRRRRRRRR]      |
|    r[RRRRRRRRRRRRRRRRRRRRRRR]     |
|    r[RRRRRRRRRRRRRRRRRRRRRRR]     |
|   r[RRRRRRRRRR]W r[RRR]W r[RRRRRRRRR]   |
|  r[RRRRRRRRRRR]  r[RRR]  r[RRRRRRRRRR]  |
| r[R RRRRR  RRRRRRRRRRRRR  RRRRRR] |
|  r[RR R RRRRR RRRRRRR  RRR   RR]  |
|    r[R     RRR      RRR]          |
|            r[RR    R]             |"#;
    aa.to_string()
}

/*

    _~^~^~_
\) /  o o  \ (/
  '_   -   _'
  / '-----' \

*/

pub fn mini_ferris_aa() -> String {
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

/*
|    CCC                 CCC     |
|   CCCCCC            CCCCCCC    |
|   CCCCCCCCCCCCCCCCCCCCCCCCC    |
|  CCCCCCCCCCCCCCCCCCCCCCCCCC    |
|  CCCWWWWWWCCCCCCCCCCWWWWWCCC   |
|  CCCWWWWWWWWWCCCCCCWWWWWWWWCC  |
|  CWWWW   WWWWCCCCWWWW   WWWWW  |
| CWWWW  WW WWWWCCCWWW  WW WWWWC |
| CWWWW     WWWWCCCWWW     WWWC  |
| CCWWWWW  WWWWC WCWWWWW  WWWWC  |
| CCCWWWWWWWWCC    CCWWWWWWWCCCC |
| CCCCCCCCCCCGGGGGGGGCCCCCCCCCCC |
|  CCCCCCCCCCGWWWWWWGCCCCCCCCCCC |
|  CCCCCCCCCCCCCCCCCCCCCCCCCCCC  |
|  CCCCCCCCCCCCCCCCCCCCCCCCCCC   |
|      CCCCCCCCCCCCCCCCCCCCCC    |
|         CCCCCCCCCCCCCC         |
*/

pub fn gopher_aa() -> String {
    #[rustfmt::skip]
    let aa = r#"
|               c[..-xZY""""""""""Y0+..]              |
|          c[..Z"!]                ..  c[."4,...JJ..]    |
|  c[.Y"77"HY^] .,77?7!..      ./^MMMM.7.  c[Ta    .W.]  |
| c[K   ..J^] .7MMMMMMMM.i    ,`MMMMMMMMM1.  c[WMN.  b]  |
|c[.|  (M#]   \````MMMMMMMt  .!`  `MMMMMMMt   c[WF  .F]  |
| c[W,  d`]  .     MMMMMMM(  .,    MMMMMMMc    c[N..Y]   |
|  c[?5j%]    t    MMMMMM.\   1   MMMMMMM.'    c[,|]     |
|    c[d]      1`MMMMMMM`^.gNaJ(+.MMMMM.?`      c[W]     |
|   c[.F]        ?7<(?7!.    M@:.  ??!          c[-|]    |
|   c[.|]              J"""""""""1              c[.|]    |
|   c[,\]              .=</7J?7,(^               c[F]    |
|   c[-}]                .` (  r                 c[@]    |
|   c[,|]                .i."i.^                 c[@]    |"#;
    /*.   c[.|                                        @]    .
    .   c[ N                                        @]    .
    .   c[ d.                                       @]    .
    .   c[ ({                                       @]    .
    . c[..?4)                                       #_?!,].
    .c[r.  -}                                       H. <.].
    .c[(..?d~                                       d 77!].
    .    c[d                                        d]    .
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
|            BBBBBBBB            |
|         BBBBBBBBBBBBBB         |
|        BB   BBBBBBBBBBB        |
|        BBBBBBBBBBBBBBBB        |
|                BBBBBBBB        |
|   BBBBBBBBBBBBBBBBBBBBB YYYY   |
|  BBBBBBBBBBBBBBBBBBBBBB YYYYY  |
|BBBBBBBBBBBBBBBBBBBBBBBB YYYYYY |
|BBBBBBBBBBBBBBBBBBBBBBB YYYYYYY |
|BBBBBBBBBB             YYYYYYYY |
|BBBBBBBB YYYYYYYYYYYYYYYYYYYYYY |
|BBBBBBBB YYYYYYYYYYYYYYYYYYYYYY |
|  BBBBB YYYYYYYYYYYYYYYYYYYYYY  |
|   BBBB YYYYYYYYYYYYYYYYYYYYY   |
|        YYYYYYYY                |
|        YYYYYYYYYYYYYYYY        |
|        YYYYYYYYYYY  YYY        |
|         YYYYYYYYYYYYYY         |
|          YYYYYYYYYYY           |
*/

pub fn python_aa() -> String {
    #[rustfmt::skip]
    let aa = r#"|            b[BBBBBBBB]            |
|         b[BBBBBBBBBBBBBB]         |
|        b[BB   BBBBBBBBBBB]        |
|        b[BBBBBBBBBBBBBBBB]        |
|                b[BBBBBBBB]        |
|   b[BBBBBBBBBBBBBBBBBBBBB] y[YYYY]   |
|  b[BBBBBBBBBBBBBBBBBBBBBB] y[YYYYY]  |
|b[BBBBBBBBBBBBBBBBBBBBBBBB] y[YYYYYY] |
|b[BBBBBBBBBBBBBBBBBBBBBBB] y[YYYYYYY] |
|b[BBBBBBBBBB]             y[YYYYYYYY] |
|b[BBBBBBBB] y[YYYYYYYYYYYYYYYYYYYYYY] |
|b[BBBBBBBB] y[YYYYYYYYYYYYYYYYYYYYYY] |
|  b[BBBBB] y[YYYYYYYYYYYYYYYYYYYYYY]  |
|   b[BBBB] y[YYYYYYYYYYYYYYYYYYYYY]   |
|        y[YYYYYYYY]                |
|        y[YYYYYYYYYYYYYYYY]        |
|        y[YYYYYYYYYYY  YYY]        |
|         y[YYYYYYYYYYYYYY]         |
|          y[YYYYYYYYYYY]           |"#;
    aa.to_string()
}
