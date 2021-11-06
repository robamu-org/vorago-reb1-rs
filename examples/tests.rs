//! Minimal blinky for the REB1 board using the HAL API
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::{
    ToggleableOutputPin, StatefulOutputPin, OutputPin, InputPin
};
use heapless::Vec;
use panic_halt as _;
use va108xx_hal::gpio::{Output, Pin, PushPull, Floating, Input};
use va108xx_hal::prelude::*;

#[allow (dead_code)]
enum TestCase {
    BlockJ10,
    BlockJ15,
    // Test input functionality
    Pa0TiedToPa1,
    TestPullup,
    TestPulldown
}

#[entry]
fn main() -> ! {
    let mut dp = va108xx::Peripherals::take().unwrap();
    let porta = dp.PORTA.split(&mut dp.SYSCONFIG);
    let portb = dp.PORTB.split(&mut dp.SYSCONFIG);
    let test_case = TestCase::Pa0TiedToPa1;
    let mut pa_vec: Vec<Pin<Output<PushPull>>, 16> = Vec::new();
    let mut output_pin: Option<Pin<Output<PushPull>>> = None;
    let mut input_pin: Option<Pin<Input<Floating>>> = None;
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
        TestCase::Pa0TiedToPa1 => {
            output_pin = Some(porta
                .pa0
                .into_push_pull_output(&mut dp.IOCONFIG, &mut dp.PORTA)
                .enable_input(&mut dp.IOCONFIG, true)
                .downgrade()
            );
            input_pin = Some(porta
                .pa1
                .into_floating_input(&mut dp.IOCONFIG)
                .downgrade()
            );
        }
    }

    loop {
        match test_case {
            TestCase::BlockJ10 | TestCase::BlockJ15 => {
                for pin in &mut pa_vec {
                    pin.toggle().ok();
                }
            }
            TestCase::Pa0TiedToPa1 => {
                let out = output_pin.as_mut().unwrap();
                out.set_high().unwrap();
                let mut state = out.is_set_high().unwrap();
                assert!(state);
                let input = input_pin.as_ref().unwrap();
                state = input.is_high().unwrap();
                assert!(state);
                out.set_low().unwrap();
                state = out.is_set_low().unwrap();
                assert!(state);
                state = input.is_low().unwrap();
                assert!(state);
            }
        }

        cortex_m::asm::delay(5_000_000);
    }
}
