#![no_std]
#![no_main]

use bsp::entry;
use defmt::info;
use defmt_rtt as _;
use embedded_hal::digital::InputPin;
use panic_probe as _;
use rp_pico as bsp;

use bsp::hal::{pac, sio::Sio};
// use rp2040_hal::gpio::{FunctionSio, Pin, PinId, PullDown, SioOutput};

mod seven_segument_display;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);

    // ピンの集合
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut display = seven_segument_display::SevenSegumentDisplay::new(
        pins.gpio18.into_push_pull_output(),
        pins.gpio17.into_push_pull_output(),
        pins.gpio16.into_push_pull_output(),
        pins.gpio19.into_push_pull_output(),
        pins.gpio12.into_push_pull_output(),
        pins.gpio13.into_push_pull_output(),
        pins.gpio14.into_push_pull_output(),
        pins.gpio15.into_push_pull_output(),
    );

    let mut btn = pins.gpio3.into_pull_up_input();

    let mut number: u8 = 0;
    let mut wait_flg = false;

    loop {
        display.set_number(&number);

        if btn.is_high().unwrap() {
            wait_flg = true;
        }

        if wait_flg && btn.is_low().unwrap() {
            if number == 9 {
                number = 0;
            } else {
                number += 1;
            }
            wait_flg = false;
            cortex_m::asm::delay(500_000);
        }
    }
}
