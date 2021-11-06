//! Minimal blinky for the REB1 board using only PAC features
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m::interrupt;
use panic_halt as _;
use va108xx_hal::{prelude::*};
use embedded_hal::digital::v2::ToggleableOutputPin;

#[entry]
fn main() -> ! {
    let mut dp = va108xx::Peripherals::take().unwrap();
    let porta = dp.PORTA.split(&mut dp.SYSCONFIG);

    let (mut led1, mut led2, mut led3) = interrupt::free(move |cs| {
        (
            porta.pa10.into_push_pull_output(cs),
            porta.pa7.into_push_pull_output(cs),
            porta.pa6.into_push_pull_output(cs)
        )
    });
    loop {
        led1.toggle().ok();
        led2.toggle().ok();
        led3.toggle().ok();
        cortex_m::asm::delay(25_000_000);
    }
}
