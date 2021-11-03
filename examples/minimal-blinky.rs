#![no_main]
#![no_std]

use cortex_m_rt::entry;
use va108xx;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let _dp = va108xx::Peripherals::take();
    loop {

    }
}
