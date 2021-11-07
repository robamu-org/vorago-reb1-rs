//! MS and Second counter implemented using the TIM0 and TIM1 peripheral
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal as hal;
use hal::{
    prelude::*,
    pac::{self, interrupt, Interrupt}
};
use core::cell::Cell;
use cortex_m::interrupt::Mutex;
use va108xx_hal::time::Hertz;

static MS_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static SEC_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = pac::Peripherals::take().unwrap();
    let mut last_ms = 0;
    rprintln!("-- Vorago system ticks using timers --");

    unsafe {
        dp.SYSCONFIG.peripheral_clk_enable.modify(|_, w| {
            w.irqsel().set_bit()
        });
        dp.SYSCONFIG.tim_clk_enable.modify(|r, w| w.bits(r.bits() | (1 << 0) | (1 << 1)));
        dp.IRQSEL.tim[0].write(|w| w.bits(0x00));
        dp.IRQSEL.tim[1].write(|w| w.bits(0x01));
    }

    let sys_clk: Hertz = 50.mhz().into();
    let cnt_ms = sys_clk.0 / 1000 - 1;
    let cnt_sec = sys_clk.0 - 1;
    unsafe {
        dp.TIM0.cnt_value.write(|w| w.bits(cnt_ms));
        dp.TIM0.rst_value.write(|w| w.bits(cnt_ms));
        dp.TIM0.ctrl.write(|w| {
            w.enable().set_bit();
            w.irq_enb().set_bit()
        });
        dp.TIM1.cnt_value.write(|w| w.bits(cnt_sec));
        dp.TIM1.rst_value.write(|w| w.bits(cnt_sec));
        dp.TIM1.ctrl.write(|w| {
            w.enable().set_bit();
            w.irq_enb().set_bit()
        });
        cortex_m::peripheral::NVIC::unmask(Interrupt::OC0);
        cortex_m::peripheral::NVIC::unmask(Interrupt::OC1);
    }
    loop {
        let current_ms = cortex_m::interrupt::free(|cs| MS_COUNTER.borrow(cs).get());
        if current_ms - last_ms >= 1000 {
             last_ms = current_ms;
             rprintln!("MS counter: {}", current_ms);
             let second = cortex_m::interrupt::free(|cs| SEC_COUNTER.borrow(cs).get());
             rprintln!("Second counter: {}", second);
        }
        cortex_m::asm::delay(10000);
    }
}

#[interrupt]
fn OC0() {
    cortex_m::interrupt::free(|cs| {
        let mut ms = MS_COUNTER.borrow(cs).get();
        ms += 1;
        MS_COUNTER.borrow(cs).set(ms);
    });
}

#[interrupt]
fn OC1() {
    cortex_m::interrupt::free(|cs| {
        let mut sec = SEC_COUNTER.borrow(cs).get();
        sec += 1;
        SEC_COUNTER.borrow(cs).set(sec);
    });
}
