#![no_std]
#![no_main]

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
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

// NOTE: 以下がサンプルプログラムからコピペ
use bsp::hal;
// 割り込み機能
use bsp::hal::pac::interrupt;
// USB Device support
use usb_device::{class_prelude::*, prelude::*};
// USB Human Interface Device (HID) Class support
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::hid_class::HIDClass;

// interruptで使うための書き方
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
static mut USB_HID: Option<HIDClass<hal::usb::UsbBus>> = None;

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

    // twichy mouseなのでピンを使わない
    // let sio = Sio::new(pac.SIO);
    // let pins = bsp::Pins::new(
    //     pac.IO_BANK0,
    //     pac.PADS_BANK0,
    //     sio.gpio_bank0,
    //     &mut pac.RESETS,
    // );

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

    loop {
        // 分かりやすいように大げさに動かす
        delay.delay_ms(1000);

        let rep = KeyboardReport {
            modifier: 0,
            reserved: 0,
            keycodes: [0x04 /* A */, 0, 0, 0, 0, 0],
        };

        push_key_event(rep).ok().unwrap_or(0);

        delay.delay_ms(1000);

        let rep = KeyboardReport {
            modifier: 0,
            reserved: 0,
            keycodes: [0x05 /* B */, 0, 0, 0, 0, 0],
        };
        push_key_event(rep).ok().unwrap_or(0);
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
