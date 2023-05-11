// キーボード固有の機能をすべてここに入れたい
use crate::bsp::hal::gpio::DynPin;
use crate::bsp::Pins;
use embedded_hal::digital::v2::{InputPin, OutputPin};

#[allow(dead_code)]
enum KeyMapping {
    K(u8),  // Key(keycode)
    KM(u8), // Key(modifier)
    // M(u8),           // Mouse(button)
    L(usize),        // Layer(layer)
    S(&'static str), // String macro(text)
    Empty,
}
use KeyMapping::*;

type KeyMapLayer = [[KeyMapping; 2]; 2];
type ScanMatrix = [[bool; 2]; 2];
fn make_scan_matrix() -> ScanMatrix {
    return [[false; 2]; 2];
}

type KeyMap = [KeyMapLayer; 2];

#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_0: KeyMapLayer = [
    [K(31),  K(32),],
    [K(33),  K(34),],
];
#[allow(dead_code)]
#[rustfmt::skip]
const LAYER_2: KeyMapLayer = [
    [K(31),  K(32),],
    [K(33),  K(34),],
];

pub const KEYMAP: KeyMap = [LAYER_0, LAYER_2];

pub struct KeyScanner {
    keymap: KeyMap,
    rows: [DynPin; 2],
    cols: [DynPin; 2],
    current_layer: usize,
    // macro_buf: [char; MACRO_BUF_SIZE],
    // macro_buf_read_index: usize,
    // macro_buf_write_index: usize,
}

impl KeyScanner {
    pub fn new(pins: Pins) -> Self {
        let row0 = pins.gpio20.into_push_pull_output();
        let row1 = pins.gpio11.into_push_pull_output();
        let mut rows: [DynPin; 2] = [row0.into(), row1.into()];

        let col0 = pins.gpio19.into_pull_up_input();
        let col1 = pins.gpio10.into_pull_up_input();
        let mut cols: [DynPin; 2] = [col1.into(), col0.into()];
        // 初期化
        rows[0].set_low().unwrap();
        rows[1].set_high().unwrap();

        KeyScanner {
            keymap: KEYMAP,
            rows: rows,
            cols: cols,
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
    fn scan(&mut self) -> KeyScanResult {
        if let Some(c) = self.macro_pop() {
            if let Some(result) = char_to_key_scan_result(c) {
                delay_ms(10);
                return result;
            }
        }
        let mut mouse_buttons = 0;
        let mut keycodes = [0u8; 6];
        let mut modifiers = 0;
        let mut next_keycode_index = 0;
        let mut number_of_keys_pressed = 0;

        let matrix = self.scan_matrix();

        let mut next_layer = 0;

        for (y, row) in matrix.iter().enumerate() {
            for (x, key) in row.iter().enumerate() {
                if !*key {
                    continue;
                }
                number_of_keys_pressed += 1;
                match self.keymap[self.current_layer][y][x] {
                    K(keycode) => {
                        if next_keycode_index < keycodes.len() {
                            keycodes[next_keycode_index] = keycode;
                            next_keycode_index += 1;
                        }
                    }
                    KM(modifier) => {
                        modifiers |= modifier;
                    }
                    M(button) => {
                        mouse_buttons |= button;
                    }
                    L(layer) => {
                        next_layer = layer;
                    }
                    S(s) => {
                        for c in s.chars() {
                            match c {
                                'A'..='Z' | '{' | '}' | '!' => {
                                    self.macro_push(1 as char);
                                }
                                _ => {
                                    self.macro_push('\0');
                                }
                            }
                            self.macro_push(c);
                            self.macro_push('\0');
                        }
                    }
                    Empty => {}
                }
            }
        }
        self.current_layer = next_layer;
        KeyScanResult {
            keycodes,
            modifiers,
            mouse_buttons,
            number_of_keys_pressed,
        }
    }
}
