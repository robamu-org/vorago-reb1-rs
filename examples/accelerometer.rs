#![no_main]
#![no_std]
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal::{
    pac::{self, interrupt},
    prelude::*,
    spi::{self, Spi},
    timer::{default_ms_irq_handler, set_up_ms_timer, Delay},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- Vorago Temperature Sensor and I2C Example --");
    let mut dp = pac::Peripherals::take().unwrap();
    let tim0 = set_up_ms_timer(
        &mut dp.SYSCONFIG,
        &mut dp.IRQSEL,
        50.mhz().into(),
        dp.TIM0,
        interrupt::OC0,
    );
    let mut delay = Delay::new(tim0);
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::OC0);
    }

    let pinsa = PinsA::new(&mut dp.SYSCONFIG, None, dp.PORTA);
    let mut spi_cfg = spi::SpiConfig::default();
    let (sck, mosi, miso) = (
        pinsa.pa20.into_funsel_1(),
        pinsa.pa19.into_funsel_1(),
        pinsa.pa18.into_funsel_1(),
    );
    let transfer_cfg = spi::TransferConfig::new(
        spi_clk, mode, hw_cs, blockmode, sod
    );
    let spi = Spi::spia(
        dp.SPIA,
        (sck, miso, mosi),
        50.mhz(),
        spi_cfg,
        Some(&mut dp.SYSCONFIG),
        None,
    );
    loop {
    }
}

#[interrupt]
fn OC0() {
    default_ms_irq_handler();
}
