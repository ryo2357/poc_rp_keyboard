#![no_std]
#![no_main]

// pub mod keycodes;
// pub mod keymap;
mod keybord;
use keybord::Keyboard;

use ae_rp2040 as bsp;
use bsp::entry;

// デバッガのクレート。デバッグピンと通信する準備は必要
use defmt::*;
use defmt_rtt as _;
// panic機能
use panic_probe as _;

// ピンのトレイト
use embedded_hal::digital::v2::{InputPin, OutputPin};

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio::DynPin,
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
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::hid_class::HIDClass;

// interruptで使うための書き方
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
static mut USB_HID: Option<HIDClass<hal::usb::UsbBus>> = None;

static DELAY: Mutex<OnceCell<RefCell<Delay>>> = Mutex::new(OnceCell::new());

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
    // [rp-hal-boards/pico_usb_twitchy_mouse.rs at main · rp-rs/rp-hal-boards · GitHub](https://github.com/rp-rs/rp-hal-boards/blob/main/boards/rp-pico/examples/pico_usb_twitchy_mouse.rs)
    // clocksはサンプルコードだとエラーが出るので

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
        .product("Twitchy Mousey")
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
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
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
    // LED pin12
    // 右スイッチ　pin19,pin20
    // 左スイッチ　pin10,pin11
    // let mut led_pin = pins.gpio12.into_push_pull_output();

    // let row0 = pins.gpio20.into_push_pull_output();
    // let row1 = pins.gpio11.into_push_pull_output();
    // let rows: [&mut DynPin; 2] = [&mut row0.into(), &mut row1.into()];

    // let col0 = pins.gpio19.into_pull_up_input();
    // let col1 = pins.gpio10.into_pull_up_input();
    // let mut cols: [&mut DynPin; 2] = [&mut col1.into(), &mut col0.into()];
    // // 初期化
    // rows[0].set_low().unwrap();
    // rows[1].set_high().unwrap();

    // キースキャナの作成
    // let mut key_scanner = KeyScanner::new(rows, cols);
    let mut keyboard = Keyboard::new(pins.into());

    // メインループの準備
    let mut last_keycodes = [0u8; 6];
    let mut last_modifiers = 0;
    // メインループ
    loop {
        let result = keyboard.scan();
        // リブート機能　⇒　とりまいらない
        // if result.number_of_keys_pressed > 5 {
        //     // Reboot the device into BOOTSEL mode when >5 keys are pressed
        //     clear_screen();
        //     print!(0, "****");
        //     print!(1, "BOOT");
        //     print!(2, "SEL");
        //     print!(3, "****");
        //     hal::rom_data::reset_to_usb_boot(0, 0);
        //     loop {
        //         cortex_m::asm::wfe();
        //     }
        // }

        let keycodes = result.keycodes;
        let modifiers = result.modifiers;

        if last_keycodes != keycodes || last_modifiers != modifiers {
            let rep = KeyboardReport {
                modifier: modifiers,
                reserved: 0,
                keycodes,
            };
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
