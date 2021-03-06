//! Blink the orange LED on pin D13

#![no_std]
#![no_main]
#![feature(used)]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate metro_m0 as hal;
extern crate panic_semihosting;
#[macro_use(entry)]
extern crate cortex_m_rt;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::{CorePeripherals, Peripherals};

entry!(main);

fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::new(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::pins(peripherals.PORT);
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        delay.delay_ms(200u8);
        red_led.set_high();
        delay.delay_ms(200u8);
        red_led.set_low();
    }
}
