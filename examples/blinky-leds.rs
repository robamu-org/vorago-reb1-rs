//! Blinky examples using the PAC directly, the HAL, or the BSP
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use embedded_hal::digital::v2::ToggleableOutputPin;
use va108xx_hal::prelude::*;
use vorago_reb1::leds::Leds;

// REB LED pin definitions. All on port A
const LED_D2: u32 = 1 << 10;
const LED_D3: u32 = 1 << 7;
const LED_D4: u32 = 1 << 6;

#[allow (dead_code)]
enum LibType {
    Pac,
    Hal,
    Bsp,
}

#[entry]
fn main() -> ! {
    let mut dp = va108xx::Peripherals::take().unwrap();
    let porta = dp.PORTA.split(&mut dp.SYSCONFIG);

    let lib_type = LibType::Bsp;

    match lib_type {
        LibType::Pac => {
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
        LibType::Hal => {
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
        LibType::Bsp => {
            let mut leds = Leds::new(porta, &mut dp.IOCONFIG, &mut dp.PORTA);
            loop {
                for _ in 0..10 {
                    // Blink all LEDs quickly
                    for led in leds.iter_mut() {
                        led.toggle();
                    }
                    cortex_m::asm::delay(5_000_000);
                }
                // Now use a wave pattern
                loop {
                    leds[0].toggle();
                    cortex_m::asm::delay(5_000_000);
                    leds[1].toggle();
                    cortex_m::asm::delay(5_000_000);
                    leds[2].toggle();
                    cortex_m::asm::delay(5_000_000);
                }
            }
        }
    }
}
