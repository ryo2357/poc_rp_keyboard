// キーボード固有の機能をすべてここに入れたい
use crate::bsp::hal::gpio::DynPin;
use crate::bsp::Pins;

use crate::{KeyMap, KeyMapLayer};

use crate::keycodes::*;

// LED pin12
// 右スイッチ　pin19,pin20
// 左スイッチ　pin10,pin11
// [右,なし]
// [なし,左]

// [US配列ライクなJISキーボードにするカスタムキーコードを作った - Qiita](https://qiita.com/koktoh/items/874be0e4d058aae54180)
// windows側はJISキーボードとしているので記号周りに工夫が必用
// mappingにshiftとキーの同時押しも定義しないとダメそう

pub const KBDSIZE_ROWS: usize = 2;
pub const KBDSIZE_COLS: usize = 2;
pub const KBDSIZE_LED: usize = 0;

pub const KBDSIZE_LAYERS: usize = 2;

pub fn set_pins(
    pins: Pins,
) -> (
    [DynPin; KBDSIZE_ROWS],
    [DynPin; KBDSIZE_COLS],
    [DynPin; KBDSIZE_LED],
) {
    let rows = [
        pins.gpio20.into_push_pull_output().into(),
        pins.gpio11.into_push_pull_output().into(),
    ];
    let cols = [
        pins.gpio19.into_pull_up_input().into(),
        pins.gpio10.into_pull_up_input().into(),
    ];
    // let leds = [pins.gpio12.into_push_pull_output().into()];
    let leds = [];
    (rows, cols, leds)
}

// keycode 0x04 : key A
// keycode 0x05 : key B

// shiftの検証
#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_0: KeyMapLayer = [
    [   A,EMPTY, ],// 2
    [   EMPTY, LSFT,],//shift
];

// shift同時押しキーの検証
// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_0: KeyMapLayer = [
//     [SK(0x1f),  Empty,],// 2
//     [Empty,  SK(0x04),],//shift
// ];

// 入力切り替えの検証
// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_0: KeyMapLayer = [
//     [K(0x04),  Empty,],// A
//     [Empty,  K(0x90),],//　Keyboard LANG1
// ];

// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_1: KeyMapLayer = [
//     [A,  EMPTY,],
//     [EMPTY,  KEY_2,],
// ];

//  キーコード作成での検証

// 1.keycode49の確認 ⇒　] }を出力
// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_0: KeyMapLayer = [
//     [K_49,  EMPTY,],// A
//     [EMPTY,  L1],//　Keyboard LANG1
// ];

// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_1: KeyMapLayer = [
//     [SK_49,  EMPTY,],
//     [EMPTY,  L1,],
// ];

// 2.keycode50の確認 ⇒　普通に50を出力
// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_0: KeyMapLayer = [
//     [R_SB,  EMPTY,],// A
//     [EMPTY,  L1],//　Keyboard LANG1
// ];

// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_1: KeyMapLayer = [
//     [R_CB,  EMPTY,],
//     [EMPTY,  L1,],
// ];

// 3.keycode100の確認 ⇒　普通に50を出力
// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_0: KeyMapLayer = [
//     [K100,  EMPTY,],// A
//     [EMPTY,  SK100],//　Keyboard LANG1
// ];

// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_1: KeyMapLayer = [
//     [EMPTY,  EMPTY,],
//     [EMPTY,  EMPTY,],
// ];

// 4.code 101: Keyboard Applicationの確認
// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_0: KeyMapLayer = [
//     [APP_K,  EMPTY,],// A
//     [EMPTY,  APP_K],//　Keyboard LANG1
// ];

// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_1: KeyMapLayer = [
//     [EMPTY,  EMPTY,],
//     [EMPTY,  EMPTY,],
// ];

// 5.code 135の確認
// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_0: KeyMapLayer = [
//     [BC_SL,  EMPTY,],// A
//     [EMPTY,  U_SCO],//　Keyboard LANG1
// ];

// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_1: KeyMapLayer = [
//     [EMPTY,  EMPTY,],
//     [EMPTY,  EMPTY,],
// ];

// 6.code 137の確認
// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_0: KeyMapLayer = [
//     [EN_M,  EMPTY,],// A
//     [EMPTY,  PIPE],//　Keyboard LANG1
// ];

// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_1: KeyMapLayer = [
//     [EMPTY,  EMPTY,],
//     [EMPTY,  EMPTY,],
// ];

// // 5.code 135の確認
// #[allow(dead_code)]
// #[rustfmt::skip]
// const LAYER_0: KeyMapLayer = [
//     [LGUI,  EMPTY,],// A
//     [EMPTY,  U_SCO],//　Keyboard LANG1
// ];

#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_1: KeyMapLayer = [
    [EMPTY,  EMPTY,],
    [EMPTY,  EMPTY,],
];
pub const KEYMAP: KeyMap = [LAYER_0, LAYER_1];
