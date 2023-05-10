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
use usbd_hid::descriptor::MouseReport;
use usbd_hid::hid_class::HIDClass;
/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;
/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
/// The USB Human Interface Device Driver (shared with the interrupt).
static mut USB_HID: Option<HIDClass<hal::usb::UsbBus>> = None;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

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

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // GPIO23を通電させている間、GPIO14のLEDを発行させる
    // GPIO23はBSPをいじっているため確認
    let mut led_pin = pins.gpio14.into_push_pull_output();
    let btn = pins.gpio23.into_pull_up_input();
    loop {
        if btn.is_low().unwrap() {
            led_pin.set_high().unwrap();
        } else {
            led_pin.set_low().unwrap();
        }
    }

    // // GPIO14を使ったLチカ
    // let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    // let mut led_pin = pins.gpio14.into_push_pull_output();
    // loop {
    //     info!("on!");
    //     led_pin.set_high().unwrap();
    //     delay.delay_ms(500);
    //     info!("off!");
    //     led_pin.set_low().unwrap();
    //     delay.delay_ms(500);
    // }
}
