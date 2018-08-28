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
extern crate embedded_graphics;
extern crate ssd1306;

use embedded_graphics::Drawing;
use embedded_graphics::{fonts::Font6x8, prelude::*};
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::{CorePeripherals, Peripherals};
use ssd1306::prelude::*;
use ssd1306::Builder;

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

    let i2c = hal::i2c_master(
        &mut clocks,
        400.khz(),
        peripherals.SERCOM3,
        &mut peripherals.PM,
        pins.sda,
        pins.scl,
        &mut pins.port,
    );

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    disp.init().unwrap();
    disp.flush().unwrap();
    disp.draw(
        Font6x8::render_str("Hello World!")
            .with_stroke(Some(1u8.into()))
            .translate(Coord::new(10, 10))
            .into_iter(),
    );
    disp.flush().unwrap();

    loop {
        delay.delay_ms(200u8);
        red_led.set_high();
        delay.delay_ms(200u8);
        red_led.set_low();
    }
}
