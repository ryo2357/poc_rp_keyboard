// キーボード固有の機能をすべてここに入れたい

use crate::bsp::hal::gpio::DynPin;
use crate::bsp::Pins;
use embedded_hal::digital::v2::{InputPin, OutputPin};

use crate::delay_ms;

// LED pin12
// 右スイッチ　pin19,pin20
// 左スイッチ　pin10,pin11
// [右,なし]
// [なし,左]

// [US配列ライクなJISキーボードにするカスタムキーコードを作った - Qiita](https://qiita.com/koktoh/items/874be0e4d058aae54180)
// windows側はJISキーボードとしているので記号周りに工夫が必用
// mappingにshiftとキーの同時押しも定義しないとダメそう

#[allow(dead_code)]
enum KeyMapping {
    K(u8),  // Key(keycode)
    KM(u8), // Key(modifier)
    // M(u8),           // Mouse(button)
    L(usize), // Layer(layer)
    // S(&'static str), // String macro(text)
    Empty,
}
use KeyMapping::*;

type KeyMapLayer = [[KeyMapping; 2]; 2];
type ScanMatrix = [[bool; 2]; 2];
fn make_scan_matrix() -> ScanMatrix {
    return [[false; 2]; 2];
}

type KeyMap = [KeyMapLayer; 2];
// keycode 0x04 : key A
// keycode 0x05 : key B

#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_0: KeyMapLayer = [
    [K(0x04),  Empty,],
    [Empty,  L(1),],
];
#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_1: KeyMapLayer = [
    [K(0x05),  Empty,],
    [Empty,  L(1),],
];

const KEYMAP: KeyMap = [LAYER_0, LAYER_1];

pub struct Keyboard {
    keymap: KeyMap,
    rows: [DynPin; 2],
    cols: [DynPin; 2],
    leds: [DynPin; 1],
    current_layer: usize,
    // macro_buf: [char; MACRO_BUF_SIZE],
    // macro_buf_read_index: usize,
    // macro_buf_write_index: usize,
}
pub struct KeyScanResult {
    pub keycodes: [u8; 6],
    pub modifiers: u8,
    // mouse_buttons: u8,
    pub number_of_keys_pressed: u8,
}

impl Keyboard {
    pub fn new(pins: Pins) -> Self {
        let row0 = pins.gpio20.into_push_pull_output();
        let row1 = pins.gpio11.into_push_pull_output();
        let mut rows: [DynPin; 2] = [row0.into(), row1.into()];

        let col0 = pins.gpio19.into_pull_up_input();
        let col1 = pins.gpio10.into_pull_up_input();
        let mut cols: [DynPin; 2] = [col0.into(), col1.into()];
        // 初期化
        rows[0].set_low().unwrap();
        rows[1].set_high().unwrap();

        let led_pin = pins.gpio12.into_push_pull_output();
        let mut leds: [DynPin; 1] = [led_pin.into()];

        Self {
            keymap: KEYMAP,
            rows: rows,
            cols: cols,
            leds: leds,
            current_layer: 0,
        }
    }

    fn scan_matrix(&mut self) -> ScanMatrix {
        let mut matrix = make_scan_matrix();
        for (y, row) in matrix.iter_mut().enumerate() {
            for (i, pin_row) in self.rows.iter_mut().enumerate() {
                if i == y {
                    pin_row.set_low().unwrap();
                } else {
                    pin_row.set_high().unwrap();
                }
            }
            delay_ms(5); // Wait a bit to propagete the voltage
            for (x, key) in row.iter_mut().enumerate() {
                *key = self.cols[x].is_low().unwrap();
            }
        }
        matrix
    }

    fn control_led(&mut self) {
        match self.current_layer {
            0 => self.leds[0].set_low().unwrap(),
            1 => self.leds[0].set_high().unwrap(),
            _ => self.leds[0].set_low().unwrap(),
        };
    }

    pub fn scan(&mut self) -> KeyScanResult {
        let mut keycodes = [0u8; 6];
        let mut modifiers = 0;
        let mut next_keycode_index = 0;
        let mut number_of_keys_pressed = 0;

        let matrix = self.scan_matrix();

        // レイヤーボタンが押されていなければレイヤー0にもどる
        let mut next_layer = 0;

        for (y, row) in matrix.iter().enumerate() {
            for (x, key) in row.iter().enumerate() {
                if !*key {
                    continue;
                }
                number_of_keys_pressed += 1;
                match self.keymap[self.current_layer][y][x] {
                    K(keycode) => {
                        // 同時押しは6個までしか入らない
                        if next_keycode_index < keycodes.len() {
                            keycodes[next_keycode_index] = keycode;
                            next_keycode_index += 1;
                        }
                    }
                    KM(modifier) => {
                        modifiers |= modifier;
                    }
                    L(layer) => {
                        next_layer = layer;
                    }
                    Empty => {}
                }
            }
        }
        self.current_layer = next_layer;
        self.control_led();

        KeyScanResult {
            keycodes,
            modifiers,
            number_of_keys_pressed,
        }
    }
}
