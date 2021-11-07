//! Minimal blinky for the REB1 board using only PAC features
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

// REB LED pin definitions. All on port A
const LED_D2: u32 = 1 << 10;
const LED_D3: u32 = 1 << 7;
const LED_D4: u32 = 1 << 6;

#[entry]
fn main() -> ! {
    let dp = va108xx::Peripherals::take().unwrap();
    // Enable all peripheral clocks
    dp.SYSCONFIG
        .peripheral_clk_enable
        .modify(|_, w| unsafe { w.bits(0xffffffff) });
    dp.PORTA
        .dir()
        .modify(|_, w| unsafe { w.bits(LED_D2 | LED_D3 | LED_D4) });
    dp.PORTA
        .datamask()
        .modify(|_, w| unsafe { w.bits(LED_D2 | LED_D3 | LED_D4) });
    loop {
        dp.PORTA
            .togout()
            .write(|w| unsafe { w.bits(LED_D2 | LED_D3 | LED_D4) });
        cortex_m::asm::delay(25_000_000);
    }
}
