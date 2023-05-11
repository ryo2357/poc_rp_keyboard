use super::keycodes::*;
use crate::keymap::KeyMapping::*;

#[allow(dead_code)]
pub enum KeyMapping {
    K(u8),  // Key(keycode)
    KM(u8), // Key(modifier)
    // M(u8),           // Mouse(button)
    L(usize),        // Layer(layer)
    S(&'static str), // String macro(text)
    Empty,
}

pub type KeyMapLayer = [[KeyMapping; 12]; 4];
pub type KeyMap = [KeyMapLayer; 3];

#[allow(dead_code)]
#[rustfmt::skip]
pub const KEYMAP_0: KeyMapLayer = [
    [K(KEY_Q),  K(KEY_W),  K(KEY_E),K(KEY_R),K(KEY_T),K(KEY_A),K(KEY_Q),  K(KEY_W),  K(KEY_E),K(KEY_R),K(KEY_T),K(KEY_A),],
    [K(KEY_A),  K(KEY_S),  K(KEY_D),K(KEY_F),K(KEY_G),K(KEY_B),K(KEY_Q),  K(KEY_W),  K(KEY_E),K(KEY_R),K(KEY_T),K(KEY_A),],
    [K(KEY_Z),  K(KEY_X),  K(KEY_C),K(KEY_V),K(KEY_B),K(KEY_C),K(KEY_Q),  K(KEY_W),  K(KEY_E),K(KEY_R),K(KEY_T),K(KEY_A),],
    [KM(LSHIFT),KM(LCTRL), K(LEFT), K(RIGHT),K(SP),   K(BS),K(KEY_Q),  K(KEY_W),  K(KEY_E),K(KEY_R),K(KEY_T),K(KEY_A),],
];
