//! Test image
//!
//! It would be nice to use a test framework like defmt-test, but I have issues
//! with probe run and it would be better to make the RTT work first
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use embedded_hal::digital::v2::{
    ToggleableOutputPin, StatefulOutputPin, OutputPin, InputPin
};
use heapless::Vec;
use panic_rtt_target as _;
use va108xx_hal::gpio::{Output, Pin, PushPull};
use va108xx_hal::prelude::*;

#[allow (dead_code)]
#[derive (Debug)]
enum TestCase {
    BlockJ10,
    BlockJ15,
    // Tie PORTA[0] to PORTA[1] for these tests!
    TestBasic,
    TestPullup,
    TestPulldown,
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- VA108xx Test Application --");
    let mut dp = va108xx::Peripherals::take().unwrap();
    let porta = dp.PORTA.split(&mut dp.SYSCONFIG);
    let portb = dp.PORTB.split(&mut dp.SYSCONFIG);
    let test_case = TestCase::TestPulldown;
    let mut pa_vec: Vec<Pin<Output<PushPull>>, 16> = Vec::new();

    match test_case {
        TestCase::TestBasic | TestCase::TestPulldown | TestCase::TestPullup => {
            rprintln!("Test case {:?}. Make sure to tie PORTA[0] to PORTA[1]", test_case);
        }
        TestCase::BlockJ10 | TestCase::BlockJ15 => {
            rprintln!("Test case {:?}", test_case);
        }
    }
    match test_case {
        TestCase::BlockJ10 => {
            let pa0 = porta
                .pa0
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
            let pa1 = porta
                .pa1
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
            let pa2 = porta
                .pa2
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
            let pa3 = porta
                .pa3
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
            let pa4 = porta
                .pa4
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
            let pa5 = porta
                .pa5
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
            let pa8 = porta
                .pa8
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
            let pa9 = porta
                .pa9
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);

            pa_vec.push(pa0.downgrade()).ok();
            pa_vec.push(pa1.downgrade()).ok();
            pa_vec.push(pa2.downgrade()).ok();
            pa_vec.push(pa3.downgrade()).ok();
            pa_vec.push(pa4.downgrade()).ok();
            pa_vec.push(pa5.downgrade()).ok();
            pa_vec.push(pa8.downgrade()).ok();
            pa_vec.push(pa9.downgrade()).ok();
        }
        TestCase::BlockJ15 => {
            let pa24 = porta
                .pa24
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
            let pa25 = porta
                .pa25
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
            let pa26 = porta
                .pa26
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
            let pa27 = porta
                .pa27
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
            // Verify PORTB is working as well
            let pb2 = portb
                .pb2
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTB);
            let pb3 = portb
                .pb3
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTB);
            pa_vec.push(pa24.downgrade()).ok();
            pa_vec.push(pa25.downgrade()).ok();
            pa_vec.push(pa26.downgrade()).ok();
            pa_vec.push(pa27.downgrade()).ok();
            pa_vec.push(pb2.downgrade()).ok();
            pa_vec.push(pb3.downgrade()).ok();
        }
        TestCase::TestBasic => {
            // Tie PORTA[0] to PORTA[1] for these tests!
            let mut out = porta
                .pa0
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA)
                .enable_input(&mut dp.IOCONFIG, true);
            let input = porta
                .pa1
                .into_floating_input(&mut dp.IOCONFIG);
            out.set_high().unwrap();
            assert!(out.is_set_high().unwrap());
            assert!(input.is_high().unwrap());
            out.set_low().unwrap();
            assert!(out.is_set_low().unwrap());
            assert!(input.is_low().unwrap());
        }
        TestCase::TestPullup => {
            // Tie PORTA[0] to PORTA[1] for these tests!
            let input = porta
                .pa1
                .into_pull_up_input(&mut dp.IOCONFIG);
            assert!(input.is_high().unwrap());
            let mut out = porta.pa0
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
            out.set_low().unwrap();
            assert!(input.is_low().unwrap());
            out.set_high().unwrap();
            assert!(input.is_high().unwrap());
            out.into_floating_input(&mut dp.IOCONFIG);
            assert!(input.is_high().unwrap());
        }
        TestCase::TestPulldown => {
            // Tie PORTA[0] to PORTA[1] for these tests!
            let input = porta
                .pa1
                .into_pull_down_input(&mut dp.IOCONFIG, &mut dp.PORTA);
            assert!(input.is_low().unwrap());
            let mut out = porta.pa0
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA);
            out.set_low().unwrap();
            assert!(input.is_low().unwrap());
            out.set_high().unwrap();
            assert!(input.is_high().unwrap());
            out.into_floating_input(&mut dp.IOCONFIG);
            assert!(input.is_low().unwrap());
        }
    }

    loop {
        match test_case {
            TestCase::BlockJ10 | TestCase::BlockJ15 => {
                for pin in &mut pa_vec {
                    pin.toggle().ok();
                }
            }
            _ => ()
        }
        cortex_m::asm::delay(5_000_000);
    }
}
