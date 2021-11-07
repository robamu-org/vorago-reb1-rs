//! Minimal blinky for the REB1 board using the HAL API
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::ToggleableOutputPin;
use panic_halt as _;
use va108xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut dp = va108xx::Peripherals::take().unwrap();
    let porta = dp.PORTA.split(&mut dp.SYSCONFIG);
    let mut led1 = porta
        .pa10
        .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
    let mut led2 = porta
        .pa7
        .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
    let mut led3 = porta
        .pa6
        .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
    loop {
        led1.toggle().ok();
        led2.toggle().ok();
        led3.toggle().ok();
        cortex_m::asm::delay(25_000_000);
    }
}
