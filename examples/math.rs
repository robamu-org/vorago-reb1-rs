//! Minimal blinky for the REB1 board using only PAC features
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let float: f32 = 3.2;
    let float_division: f32 = 3.2 / 1.6;
    let float_floored = float_division.floor();

    let sinus_of_four = sin(4);
    loop {

    }
}
