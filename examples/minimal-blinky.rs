#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let dp = va108xx::Peripherals::take().unwrap();
    // Enable all peripheral clocks
    dp.SYSCONFIG.peripheral_clk_enable.modify(|_, w| unsafe {
        w.bits(0xffffffff)
    });
    loop {

    }
}
