// [hikalium/keyball-rs](https://github.com/hikalium/keyball-rs)
// 上記リポジトリを参考に作成
#![no_std]
#![no_main]

mod layout;
use layout::{set_pins, KBDSIZE_COLS, KBDSIZE_LAYERS, KBDSIZE_LED, KBDSIZE_ROWS, KEYMAP};
mod jis_keycodes;
use jis_keycodes as keycodes;

use ae_rp2040 as bsp;
use bsp::entry;

// デバッガのクレート。デバッグピンと通信する準備は必要
// use defmt::*;
use defmt_rtt as _;
// panic機能
use panic_probe as _;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

// NOTE: 以下がサンプルプログラムからコピペX
use bsp::hal;
// 割り込み機能
use bsp::hal::pac::interrupt;
// ディレイ機能
use core::cell::RefCell;
use cortex_m::delay::Delay;
use cortex_m::interrupt::CriticalSection;
use cortex_m::interrupt::Mutex;
use once_cell::unsync::OnceCell;
// USB Device support
use usb_device::{class_prelude::*, prelude::*};
// USB Human Interface Device (HID) Class support
use crate::bsp::hal::gpio::DynPin;
use crate::bsp::Pins;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::hid_class::HIDClass;

// interruptで使うための書き方
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
static mut USB_HID: Option<HIDClass<hal::usb::UsbBus>> = None;

static DELAY: Mutex<OnceCell<RefCell<Delay>>> = Mutex::new(OnceCell::new());

// キーボードのセッティング関係
#[allow(dead_code)]
pub enum KeyMapping {
    K(u8),  // Key(keycode)
    SK(u8), // Left Shiftとの組み合わせ入力
    KM(u8), // Key(modifier)
    // M(u8),           // Mouse(button)
    L(usize), // Layer(layer)
    // S(&'static str), // String macro(text)
    Empty,
}
use KeyMapping::*;
pub type KeyMap = [KeyMapLayer; KBDSIZE_LAYERS];
pub type KeyMapLayer = [[KeyMapping; KBDSIZE_COLS]; KBDSIZE_ROWS];
pub type ScanMatrix = [[bool; KBDSIZE_COLS]; KBDSIZE_ROWS];

fn make_scan_matrix() -> ScanMatrix {
    return [[false; KBDSIZE_COLS]; KBDSIZE_ROWS];
}

pub struct Keyboard<F> {
    keymap: KeyMap,
    rows: [DynPin; KBDSIZE_ROWS],
    cols: [DynPin; KBDSIZE_COLS],
    leds: [DynPin; KBDSIZE_LED],
    current_layer: usize,
    // macro_buf: [char; MACRO_BUF_SIZE],
    // macro_buf_read_index: usize,
    // macro_buf_write_index: usize,
    delay_ms_fn: F,
}
pub struct KeyScanResult {
    pub keycodes: [u8; 6],
    pub modifiers: u8,
    // mouse_buttons: u8,
    pub number_of_keys_pressed: u8,
}

impl<F> Keyboard<F>
where
    F: Fn(u32),
{
    pub fn new(pins: Pins, delay_ms_fn: F) -> Self {
        let (rows, cols, leds) = set_pins(pins);
        Self {
            keymap: KEYMAP,
            rows: rows,
            cols: cols,
            leds: leds,
            current_layer: 0,
            delay_ms_fn: delay_ms_fn,
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
            (self.delay_ms_fn)(5); // Wait a bit to propagete the voltage
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
                    SK(keycode) => {
                        // 同時押しは6個までしか入らない
                        if next_keycode_index < keycodes.len() {
                            keycodes[next_keycode_index] = keycode;
                            next_keycode_index += 1;
                        }
                        // Left shiftの入力
                        // pub const LSHIFT: u8 = 0b00000010;
                        modifiers |= 0b00000010;
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
        // レイヤーとLEDを合わせる
        // self.control_led();

        KeyScanResult {
            keycodes,
            modifiers,
            number_of_keys_pressed,
        }
    }
}

// KeyboardReport ではうまいこと動かない
// ledsのフィールドがいらない
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        (usage_page = KEYBOARD, usage_min = 0xE0, usage_max = 0xE7) = {
            #[packed_bits 8] #[item_settings data,variable,absolute] modifier=input;
        };
        (usage_min = 0x00, usage_max = 0xFF) = {
            #[item_settings constant,variable,absolute] reserved=input;
        };
        (usage_page = KEYBOARD, usage_min = 0x00, usage_max = 0xDD) = {
            #[item_settings data,array,absolute] keycodes=input;
        };
}
)]
#[allow(dead_code)]
pub struct KeyboardReport {
    pub modifier: u8,
    pub reserved: u8,
    pub keycodes: [u8; 6],
}

#[entry]
fn main() -> ! {
    // プログラム初期化。基本的にテンプレートのまま

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // USBドライバーのセットアップ
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    unsafe {
        USB_BUS = Some(usb_bus);
    }
    // USB　BUSアロケータの取得
    // USBデバイス・コントローラの一部で、USBバスの帯域幅を割り当てる機能
    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    // USB HIDクラスの作成　⇒　キーボードで作成
    let usb_hid = HIDClass::new(bus_ref, KeyboardReport::desc(), 60);
    unsafe {
        USB_HID = Some(usb_hid);
    }

    // USBデバイスのVIDとPIDを設定
    let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x16c0, 0x27da))
        .manufacturer("Fake company")
        .product("KEYBOARD")
        .serial_number("TEST")
        .device_class(0)
        .build();
    unsafe {
        USB_DEVICE = Some(usb_dev);
    }

    // interrupt を可能にする
    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };
    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    unsafe {
        let cs = CriticalSection::new();
        let global_delay = DELAY.borrow(&cs);
        global_delay
            .set(RefCell::new(delay))
            .map_err(|_| 0)
            .unwrap();
    }

    let sio = Sio::new(pac.SIO);
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // ハードウェア制御構造体を作成
    let mut keyboard = Keyboard::new(pins, delay_ms);

    // メインループの準備
    let mut last_keycodes = [0u8; 6];
    let mut last_modifiers = 0;
    // メインループ
    loop {
        let result = keyboard.scan();

        let keycodes = result.keycodes;
        let modifiers = result.modifiers;

        if last_keycodes != keycodes || last_modifiers != modifiers {
            let rep = KeyboardReport {
                modifier: modifiers,
                reserved: 0,
                keycodes,
            };
            // 以下の処理タイミングが何とも言えないので変更しない
            push_key_event(rep).ok().unwrap_or(0);
            last_keycodes = keycodes;
            last_modifiers = modifiers;
        }
    }
}

// critical_sectionはマルチスレッドで同期処理を行うためのクレート
fn push_key_event(report: KeyboardReport) -> Result<usize, usb_device::UsbError> {
    critical_section::with(|_| unsafe {
        // Now interrupts are disabled, grab the global variable and, if
        // available, send it a HID report
        USB_HID.as_mut().map(|hid| hid.push_input(&report))
    })
    .unwrap()
}

// 送信をinterruptで処理している
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    // Handle USB request
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();
    usb_dev.poll(&mut [usb_hid]);
}

pub fn delay_ms(ms: u32) {
    let cs = unsafe { CriticalSection::new() };
    let delay = &mut *DELAY.borrow(&cs).get().unwrap().borrow_mut();
    delay.delay_ms(ms);
}
