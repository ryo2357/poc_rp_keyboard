// [USB HID Usage ID の Scancode 変換と対応するキー | capyBaral](https://bsakatu.net/doc/usb-hid-to-scancode/)
// [USBキーボードのキーコード](http://www2d.biglobe.ne.jp/~msyk/keyboard/layout/usbkeycode.html)
// [英語の記号・マークの読み方 | 英語＠めもらんだむ](https://memotec.net/etc/mark.html)

#![allow(dead_code)]
use crate::KeyMapping;
// LAYER KEY
pub const L1: KeyMapping = KeyMapping::L(1);
pub const L2: KeyMapping = KeyMapping::L(2);
pub const L3: KeyMapping = KeyMapping::L(3);
pub const L4: KeyMapping = KeyMapping::L(4);

// Empty
pub const EMPTY: KeyMapping = KeyMapping::Empty;
pub const NO_SW: KeyMapping = KeyMapping::Empty;

// modifirキー
pub const LCTRL: KeyMapping = KeyMapping::KM(0b00000001);
pub const LSFT: KeyMapping = KeyMapping::KM(0b00000010);
pub const LALT: KeyMapping = KeyMapping::KM(0b00000100);
pub const LGUI: KeyMapping = KeyMapping::KM(0b00001000);
pub const RCTRL: KeyMapping = KeyMapping::KM(0b00010000);
pub const RSFT: KeyMapping = KeyMapping::KM(0b00100000);
pub const RALT: KeyMapping = KeyMapping::KM(0b01000000);
pub const RGUI: KeyMapping = KeyMapping::KM(0b10000000);

// code 0: Reserved (no event indicated)
// code 1: Keyboard ErrorRollOver
// code 2: Keyboard POSTFail
// code 3: Keyboard ErrorUndefined
// code 4-29: Keyboard a and A-Z
pub const A: KeyMapping = KeyMapping::K(4);
pub const B: KeyMapping = KeyMapping::K(5);
pub const C: KeyMapping = KeyMapping::K(6);
pub const D: KeyMapping = KeyMapping::K(7);
pub const E: KeyMapping = KeyMapping::K(8);
pub const F: KeyMapping = KeyMapping::K(9);
pub const G: KeyMapping = KeyMapping::K(10);
pub const H: KeyMapping = KeyMapping::K(11);
pub const I: KeyMapping = KeyMapping::K(12);
pub const J: KeyMapping = KeyMapping::K(13);
pub const K: KeyMapping = KeyMapping::K(14);
pub const L: KeyMapping = KeyMapping::K(15);
pub const M: KeyMapping = KeyMapping::K(16);
pub const N: KeyMapping = KeyMapping::K(17);
pub const O: KeyMapping = KeyMapping::K(18);
pub const P: KeyMapping = KeyMapping::K(19);
pub const Q: KeyMapping = KeyMapping::K(20);
pub const R: KeyMapping = KeyMapping::K(21);
pub const S: KeyMapping = KeyMapping::K(22);
pub const T: KeyMapping = KeyMapping::K(23);
pub const U: KeyMapping = KeyMapping::K(24);
pub const V: KeyMapping = KeyMapping::K(25);
pub const W: KeyMapping = KeyMapping::K(26);
pub const X: KeyMapping = KeyMapping::K(27);
pub const Y: KeyMapping = KeyMapping::K(28);
pub const Z: KeyMapping = KeyMapping::K(29);

// code 30: Keyboard 1 and !, JIS109: 1 !
/// 1
pub const KEY_1: KeyMapping = KeyMapping::K(30);
/// !
pub const EXCL: KeyMapping = KeyMapping::SK(30);

// code 31: Keyboard 2 and @, JIS109: 2 "
pub const KEY_2: KeyMapping = KeyMapping::K(31);
pub const D_QT: KeyMapping = KeyMapping::SK(31);

// code 32: Keyboard 3 and #, JIS109: 3 #
pub const KEY_3: KeyMapping = KeyMapping::K(32);
pub const HASH: KeyMapping = KeyMapping::SK(32);

// code 33: Keyboard 4 and $, JIS109: 4 $
pub const KEY_4: KeyMapping = KeyMapping::K(33);
pub const DOLL: KeyMapping = KeyMapping::SK(33);

// code 34: Keyboard 5 and %, JIS109: 5 %
pub const KEY_5: KeyMapping = KeyMapping::K(34);
pub const PCNT: KeyMapping = KeyMapping::SK(34);

// code 35: Keyboard 6 and ^, JIS109: 6 &
pub const KEY_6: KeyMapping = KeyMapping::K(35);
pub const AND: KeyMapping = KeyMapping::SK(35);

// code 36: Keyboard 7 and &, JIS109: 7 '
pub const KEY_7: KeyMapping = KeyMapping::K(36);
pub const S_QT: KeyMapping = KeyMapping::SK(36);

// code 37: Keyboard 8 and *, JIS109: 8 (
pub const KEY_8: KeyMapping = KeyMapping::K(37);
pub const L_RB: KeyMapping = KeyMapping::SK(37);

// code 38: Keyboard 9 and (, JIS109: 9 )
pub const KEY_9: KeyMapping = KeyMapping::K(38);
pub const R_RB: KeyMapping = KeyMapping::SK(38);

// code 39: Keyboard 0 and ), JIS109: 0
pub const KEY_0: KeyMapping = KeyMapping::K(39);

// code 40: Keyboard Return (ENTER), JIS109: Enter
pub const ENTER: KeyMapping = KeyMapping::K(40);

// code 41: Keyboard ESCAPE, JIS109: Esc
pub const ESC: KeyMapping = KeyMapping::K(41);

// code 42: Keyboard DELETE (Backspace), JIS109: Backspace
pub const BACKS: KeyMapping = KeyMapping::K(42);

// code 43: Keyboard Tab, JIS109: Tab
pub const TAB: KeyMapping = KeyMapping::K(43);

// code 44: Keyboard Spacebar, JIS109: Spacebar
pub const SPACE: KeyMapping = KeyMapping::K(44);

// code 45: Keyboard - and (underscore), JIS109: - =
pub const HYPN: KeyMapping = KeyMapping::K(45);
pub const EQUAL: KeyMapping = KeyMapping::SK(45);

// code 46: Keyboard = and +, JIS109: ^ ~
pub const HAT: KeyMapping = KeyMapping::K(46);
pub const TILDE: KeyMapping = KeyMapping::SK(46);

// code 47: Keyboard [ and {, JIS109: @ `
pub const AT_M: KeyMapping = KeyMapping::K(47);
pub const BK_QT: KeyMapping = KeyMapping::SK(47);

// code 48: Keyboard ] and }, JIS109: [ {
pub const L_SB: KeyMapping = KeyMapping::K(48);
pub const L_CB: KeyMapping = KeyMapping::SK(48);

// code 49: Keyboard \ and |, JIS109: -----
// pub const K_49: KeyMapping = KeyMapping::K(49);
// pub const SK_49: KeyMapping = KeyMapping::SK(49);
// JISキーボードには存在しない ⇒なぜか code 50 JIS109: ] }が入力

// code 50: Keyboard Non-US # and ~, JIS109: ] }
pub const R_SB: KeyMapping = KeyMapping::K(50);
pub const R_CB: KeyMapping = KeyMapping::SK(50);

// code 51: Keyboard ; and :, JIS109: ; +
pub const S_CLN: KeyMapping = KeyMapping::K(51);
pub const PLUS: KeyMapping = KeyMapping::SK(51);

// code 52: Keyboard ' and ", JIS109: : *
pub const COLON: KeyMapping = KeyMapping::K(52);
pub const STAR: KeyMapping = KeyMapping::K(52);

// code 53: Keyboard Grave Accent and Tilde
// 半角/全角キー
pub const IME: KeyMapping = KeyMapping::K(53);

// code 54: Keyboard , and <, JIS109: , <
pub const COMMA: KeyMapping = KeyMapping::K(54);
pub const L_AB: KeyMapping = KeyMapping::SK(54);

// code 55: Keyboard . and >, JIS109: . >
pub const DOT: KeyMapping = KeyMapping::K(55);
pub const R_AB: KeyMapping = KeyMapping::SK(55);

// code 56: Keyboard / and ?, JIS109: / ?
pub const SLASH: KeyMapping = KeyMapping::K(56);
pub const QST_M: KeyMapping = KeyMapping::SK(56);

// code 57: Keyboard Caps Lock, JIS109: 英数 Caps Lock
pub const EISUU: KeyMapping = KeyMapping::K(57);
pub const CPS_L: KeyMapping = KeyMapping::SK(57);

// code 58 - 69: Keyboard F1 - F12
pub const F1: KeyMapping = KeyMapping::K(58);
pub const F2: KeyMapping = KeyMapping::K(59);
pub const F3: KeyMapping = KeyMapping::K(60);
pub const F4: KeyMapping = KeyMapping::K(61);
pub const F5: KeyMapping = KeyMapping::K(62);
pub const F6: KeyMapping = KeyMapping::K(63);
pub const F7: KeyMapping = KeyMapping::K(64);
pub const F8: KeyMapping = KeyMapping::K(65);
pub const F9: KeyMapping = KeyMapping::K(66);
pub const F10: KeyMapping = KeyMapping::K(67);
pub const F11: KeyMapping = KeyMapping::K(68);
pub const F12: KeyMapping = KeyMapping::K(69);

// code 70: Keyboard PrintScreen, JIS109: Print Screen
pub const PRT_S: KeyMapping = KeyMapping::K(70);

// code 71: Keyboard Scroll Lock, JIS109: Scroll Lock
pub const SCR_L: KeyMapping = KeyMapping::K(71);

// code 72: Keyboard Pause, JIS109: Pause
pub const PAUSE: KeyMapping = KeyMapping::K(72);

// code 73: Keyboard Insert, JIS109: Insert
pub const INSRT: KeyMapping = KeyMapping::K(73);

// code 74: Keyboard Home, JIS109: Home
pub const HOME: KeyMapping = KeyMapping::K(74);

// code 75: Keyboard PageUp, JIS109: Page Up
pub const PG_U: KeyMapping = KeyMapping::K(75);

// code 76: Keyboard Delete Forward, JIS109: Delete
pub const DEL: KeyMapping = KeyMapping::K(76);

// code 77: Keyboard End, JIS109: End
pub const EMD: KeyMapping = KeyMapping::K(77);

// code 78: Keyboard PageDown, JIS109: Page Down
pub const PG_D: KeyMapping = KeyMapping::K(78);

// code 79: Keyboard RightArrow, JIS109: →
pub const RIGHT: KeyMapping = KeyMapping::K(79);

// code 80: Keyboard LeftArrow, JIS109: ←
pub const LEFT: KeyMapping = KeyMapping::K(80);

// code 81: Keyboard DownArrow, JIS109: ↓
pub const DOWN: KeyMapping = KeyMapping::K(81);

// code 82: Keyboard UpArrow, JIS109: ↑
pub const UP: KeyMapping = KeyMapping::K(82);

// code 83: Keypad Num Lock and Clear, JIS109: Num Lock
pub const NUM_L: KeyMapping = KeyMapping::K(83);

// code 84: Keypad /, JIS109: Keypad /
pub const PD_SL: KeyMapping = KeyMapping::K(84);

// code 85: Keypad *, JIS109: Keypad *
pub const PD_ST: KeyMapping = KeyMapping::K(85);

// code 86: Keypad -, JIS109: Keypad -
pub const PD_MN: KeyMapping = KeyMapping::K(86);

// code 87: Keypad +, JIS109: Keypad +
pub const PD_PS: KeyMapping = KeyMapping::K(87);

// code 88: Keypad ENTER, JIS109: Keypad Enter
pub const PD_ET: KeyMapping = KeyMapping::K(88);

// code 89～98:  Keypad 1～0
pub const PD_1: KeyMapping = KeyMapping::K(89);
pub const PD_2: KeyMapping = KeyMapping::K(90);
pub const PD_3: KeyMapping = KeyMapping::K(91);
pub const PD_4: KeyMapping = KeyMapping::K(92);
pub const PD_5: KeyMapping = KeyMapping::K(93);
pub const PD_6: KeyMapping = KeyMapping::K(94);
pub const PD_7: KeyMapping = KeyMapping::K(95);
pub const PD_8: KeyMapping = KeyMapping::K(96);
pub const PD_9: KeyMapping = KeyMapping::K(97);
pub const PD_0: KeyMapping = KeyMapping::K(98);

// code 99: Keypad . and Delete, JIS109: Keypad .
pub const PD_DT: KeyMapping = KeyMapping::K(99);

// code 100: Keyboard Non-US \ and |
// JISだと何も入力されない
// pub const K100: KeyMapping = KeyMapping::K(100);
// pub const SK100: KeyMapping = KeyMapping::SK(100);

// code 101: Keyboard Application, JIS109: Application
// 右クリックのメニューが開く。メニューキー
pub const APP_K: KeyMapping = KeyMapping::K(101);

// code 102: Keyboard Power
pub const POWER: KeyMapping = KeyMapping::K(102);

// code 103: Keypad =, JIS109: -----
// pub const K103: KeyMapping = KeyMapping::K(103);

// code 104: Keyboard F13, JIS109: -----
// pub const K104: KeyMapping = KeyMapping::K(104);
// code 105: Keyboard F14, JIS109: -----
// pub const K105: KeyMapping = KeyMapping::K(105);
// code 106: Keyboard F15, JIS109: -----
// pub const K106: KeyMapping = KeyMapping::K(106);

// code 107 - 115: Keyboard F16 - F24
pub const F16: KeyMapping = KeyMapping::K(107);
pub const F17: KeyMapping = KeyMapping::K(108);
pub const F18: KeyMapping = KeyMapping::K(109);
pub const F19: KeyMapping = KeyMapping::K(110);
pub const F20: KeyMapping = KeyMapping::K(111);
pub const F21: KeyMapping = KeyMapping::K(112);
pub const F22: KeyMapping = KeyMapping::K(113);
pub const F23: KeyMapping = KeyMapping::K(114);
pub const F24: KeyMapping = KeyMapping::K(115);

// 以下ちょっと分からないキーボードの機能、必要なものを有効かしていく

// code 116: Keyboard Execute
// pub const : KeyMapping = KeyMapping::K(116);
// code 117: Keyboard Help
// pub const : KeyMapping = KeyMapping::K(117);
// code 118: Keyboard Menu
// pub const : KeyMapping = KeyMapping::K(118);
// code 119: Keyboard Select
// pub const : KeyMapping = KeyMapping::K(119);
// code 120: Keyboard Stop
// pub const : KeyMapping = KeyMapping::K(120);
// code 121: Keyboard Again
// pub const : KeyMapping = KeyMapping::K(121);
// code 122: Keyboard Undo
// pub const : KeyMapping = KeyMapping::K(122);
// code 123: Keyboard Cut
// pub const : KeyMapping = KeyMapping::K(123);
// code 124: Keyboard Copy
// pub const : KeyMapping = KeyMapping::K(124);
// code 125: Keyboard Paste
// pub const : KeyMapping = KeyMapping::K(125);
// code 126: Keyboard Find
// pub const : KeyMapping = KeyMapping::K(126);
// code 127: Keyboard Mute
// pub const : KeyMapping = KeyMapping::K(127);
// code 128: Keyboard Volume Up
// pub const : KeyMapping = KeyMapping::K(128);
// code 129: Keyboard Volume Down
// pub const : KeyMapping = KeyMapping::K(129);
// code 130: Keyboard Locking Caps Lock
// pub const : KeyMapping = KeyMapping::K(130);
// code 131: Keyboard Locking Numb Lock
// pub const : KeyMapping = KeyMapping::K(131);
// code 132: Keyboard Locking Scroll Lock
// pub const : KeyMapping = KeyMapping::K(132);
// code 133: Keypad Comma, JIS109: -----
// pub const : KeyMapping = KeyMapping::K(133);
// code 134: Keypad Equal Sign
// pub const : KeyMapping = KeyMapping::K(134);

// code 135: Keyboard International1, JIS109: ＼ _
// ＼=￥　表示環境次第
pub const BC_SL: KeyMapping = KeyMapping::K(135);
pub const U_SCO: KeyMapping = KeyMapping::SK(135);

// code 136: Keyboard International2, JIS109: ひらがな カタカナ
// pub const : KeyMapping = KeyMapping::K(136);

// code 137: Keyboard International3, JIS109: ￥ ｜
// ＼=￥　表示環境次第
pub const EN_M: KeyMapping = KeyMapping::K(137);
pub const PIPE: KeyMapping = KeyMapping::SK(137);

// code 138: Keyboard International4, JIS109: 変換
// pub const : KeyMapping = KeyMapping::K(138);
// code 139: Keyboard International5, JIS109: 無変換
// pub const : KeyMapping = KeyMapping::K(139);
// code 140: Keyboard International6
// pub const : KeyMapping = KeyMapping::K(140);
// code 141: Keyboard International7
// pub const : KeyMapping = KeyMapping::K(141);
// code 142: Keyboard International8
// pub const : KeyMapping = KeyMapping::K(142);
// code 143: Keyboard International9
// pub const : KeyMapping = KeyMapping::K(143);

// code 144: Keyboard LANG1, JIS109: -----
// 日本語入力
pub const LANG1: KeyMapping = KeyMapping::K(144);

// code 145: Keyboard LANG2, JIS109: -----
// 英語入力
pub const LANG2: KeyMapping = KeyMapping::K(145);

// code 146: Keyboard LANG3
// pub const : KeyMapping = KeyMapping::K(146);
// code 147: Keyboard LANG4
// pub const : KeyMapping = KeyMapping::K(147);
// code 148: Keyboard LANG5
// pub const : KeyMapping = KeyMapping::K(148);
// code 149: Keyboard LANG6
// pub const : KeyMapping = KeyMapping::K(149);
// code 150: Keyboard LANG7
// pub const : KeyMapping = KeyMapping::K(150);
// code 151: Keyboard LANG8
// pub const : KeyMapping = KeyMapping::K(151);
// code 152: Keyboard LANG9
// pub const : KeyMapping = KeyMapping::K(152);
// code 153: Keyboard Alternate Erase
// pub const : KeyMapping = KeyMapping::K(153);
// code 154: Keyboard SysReq/Attention
// pub const : KeyMapping = KeyMapping::K(154);
// code 155: Keyboard Cancel
// pub const : KeyMapping = KeyMapping::K(155);
// code 156: Keyboard Clear
// pub const : KeyMapping = KeyMapping::K(156);
// code 157: Keyboard Prior
// pub const : KeyMapping = KeyMapping::K(157);
// code 158: Keyboard Return
// pub const : KeyMapping = KeyMapping::K(158);
// code 159: Keyboard Separator
// pub const : KeyMapping = KeyMapping::K(159);
// code 160: Keyboard Out
// pub const : KeyMapping = KeyMapping::K(160);
// code 161: Keyboard Oper
// pub const : KeyMapping = KeyMapping::K(161);
// code 162: Keyboard Clear/Again
// pub const : KeyMapping = KeyMapping::K(162);
// code 163: Keyboard CrSel/Props
// pub const : KeyMapping = KeyMapping::K(163);
// code 164: Keyboard ExSel
// pub const : KeyMapping = KeyMapping::K(164);
// code 165: Reserved
// pub const : KeyMapping = KeyMapping::K(165);
// code 166: Reserved
// pub const : KeyMapping = KeyMapping::K(166);
// code 167: Reserved
// pub const : KeyMapping = KeyMapping::K(167);
// code 168: Reserved
// pub const : KeyMapping = KeyMapping::K(168);
// code 169: Reserved
// pub const : KeyMapping = KeyMapping::K(169);
// code 170: Reserved
// pub const : KeyMapping = KeyMapping::K(170);
// code 171: Reserved
// pub const : KeyMapping = KeyMapping::K(171);
// code 172: Reserved
// pub const : KeyMapping = KeyMapping::K(172);
// code 173: Reserved
// pub const : KeyMapping = KeyMapping::K(173);
// code 174: Reserved
// pub const : KeyMapping = KeyMapping::K(174);
// code 175: Reserved
// pub const : KeyMapping = KeyMapping::K(175);
// code 176: Keypad 00
// pub const : KeyMapping = KeyMapping::K(176);
// code 177: Keypad 000
// pub const : KeyMapping = KeyMapping::K(177);
// code 178: Thousands Separator
// pub const : KeyMapping = KeyMapping::K(178);
// code 179: Decimal Separator
// pub const : KeyMapping = KeyMapping::K(179);
// code 180: Currency Unit
// pub const : KeyMapping = KeyMapping::K(180);
// code 181: Currency Sub-unit
// pub const : KeyMapping = KeyMapping::K(181);
// code 182: Keypad (
// pub const : KeyMapping = KeyMapping::K(182);
// code 183: Keypad )
// pub const : KeyMapping = KeyMapping::K(183);
// code 184: Keypad {, JIS109: -----
// pub const : KeyMapping = KeyMapping::K(184);
// code 185: Keypad }
// pub const : KeyMapping = KeyMapping::K(185);
// code 186: Keypad Tab
// pub const : KeyMapping = KeyMapping::K(186);
// code 187: Keypad Backspace
// pub const : KeyMapping = KeyMapping::K(187);
// code 188: Keypad A
// pub const : KeyMapping = KeyMapping::K(188);
// code 189: Keypad B
// pub const : KeyMapping = KeyMapping::K(189);
// code 190: Keypad C
// pub const : KeyMapping = KeyMapping::K(190);
// code 191: Keypad D
// pub const : KeyMapping = KeyMapping::K(191);
// code 192: Keypad E
// pub const : KeyMapping = KeyMapping::K(192);
// code 193: Keypad F
// pub const : KeyMapping = KeyMapping::K(193);
// code 194: Keypad XOR
// pub const : KeyMapping = KeyMapping::K(194);
// code 195: Keypad ^
// pub const : KeyMapping = KeyMapping::K(195);
// code 196: Keypad %
// pub const : KeyMapping = KeyMapping::K(196);
// code 197: Keypad <
// pub const : KeyMapping = KeyMapping::K(197);
// code 198: Keypad >
// pub const : KeyMapping = KeyMapping::K(198);
// code 199: Keypad &
// pub const : KeyMapping = KeyMapping::K(199);
// code 200: Keypad &&
// pub const : KeyMapping = KeyMapping::K(200);
// code 201: Keypad |
// pub const : KeyMapping = KeyMapping::K(201);
// code 202: Keypad ||
// pub const : KeyMapping = KeyMapping::K(202);
// code 203: Keypad :
// pub const : KeyMapping = KeyMapping::K(203);
// code 204: Keypad #
// pub const : KeyMapping = KeyMapping::K(204);
// code 205: Keypad Space
// pub const : KeyMapping = KeyMapping::K(205);
// code 206: Keypad @
// pub const : KeyMapping = KeyMapping::K(206);
// code 207: Keypad !
// pub const : KeyMapping = KeyMapping::K(207);
// code 208: Keypad Memory Store
// pub const : KeyMapping = KeyMapping::K(208);
// code 209: Keypad Memory Recall
// pub const : KeyMapping = KeyMapping::K(209);
// code 210: Keypad Memory Clear
// pub const : KeyMapping = KeyMapping::K(210);
// code 211: Keypad Memory Add
// pub const : KeyMapping = KeyMapping::K(211);
// code 212: Keypad Memory Subtract
// pub const : KeyMapping = KeyMapping::K(212);
// code 213: Keypad Memory Multiply
// pub const : KeyMapping = KeyMapping::K(213);
// code 214: Keypad Memory Divide
// pub const : KeyMapping = KeyMapping::K(214);
// code 215: Keypad +/-
// pub const : KeyMapping = KeyMapping::K(215);
// code 216: Keypad Clear
// pub const : KeyMapping = KeyMapping::K(216);
// code 217: Keypad Clear Entry
// pub const : KeyMapping = KeyMapping::K(217);
// code 218: Keypad Binary
// pub const : KeyMapping = KeyMapping::K(218);
// code 219: Keypad Octal
// pub const : KeyMapping = KeyMapping::K(219);
// code 220: Keypad Decimal
// pub const : KeyMapping = KeyMapping::K(220);
// code 221: Keypad Hexadecimal
// pub const : KeyMapping = KeyMapping::K(221);
// code 222: Reserved
// pub const : KeyMapping = KeyMapping::K(222);
// code 223: Reserved
// pub const : KeyMapping = KeyMapping::K(223);
// code 224: Keyboard LeftControl, JIS109: Left Ctrl
// pub const : KeyMapping = KeyMapping::K(224);
// code 225: Keyboard LeftShift, JIS109: Left Shift
// pub const : KeyMapping = KeyMapping::K(225);
// code 226: Keyboard LeftAlt, JIS109: -----
// pub const : KeyMapping = KeyMapping::K(226);

// code 227: Keyboard Left GUI, JIS109: Left Windows
// pub const WIN: KeyMapping = KeyMapping::K(227);
// modifiyキーでいいような気がする

// code 228: Keyboard RightControl, JIS109: Right Ctrl
// pub const : KeyMapping = KeyMapping::K(228);
// code 229: Keyboard RightShift, JIS109: Right Shift
// pub const : KeyMapping = KeyMapping::K(229);
// code 230: Keyboard RightAlt, JIS109: Right Alt
// pub const : KeyMapping = KeyMapping::K(230);
// code 231: Keyboard Right GUI, JIS109: Right Windows
// pub const : KeyMapping = KeyMapping::K(231);
