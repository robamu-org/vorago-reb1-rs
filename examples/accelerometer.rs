#![no_main]
#![no_std]
use cortex_m_rt::entry;
use embedded_hal::spi::MODE_3;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal::{
    gpio::PinsA,
    pac::{self, interrupt},
    prelude::*,
    spi::{self, Spi},
    timer::{default_ms_irq_handler, set_up_ms_timer, Delay},
};

const READ_MASK: u8 = 1 << 7;
const MULTI_BYTE_MASK: u8 = 1 << 6;
const DEVID_REG: u8 = 0x00;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- Vorago Accelerometer Example --");
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
    let spi_cfg = spi::SpiConfig::default();
    let (sck, mosi, miso) = (
        pinsa.pa20.into_funsel_2(),
        pinsa.pa19.into_funsel_2(),
        pinsa.pa18.into_funsel_2(),
    );
    let transfer_cfg = spi::TransferConfig::new(
        1.mhz(),
        MODE_3,
        Some(pinsa.pa16.into_funsel_2()),
        true,
        false,
    );
    let mut spi = Spi::spib(
        dp.SPIB,
        (sck, miso, mosi),
        50.mhz(),
        spi_cfg,
        Some(&mut dp.SYSCONFIG),
        Some(&transfer_cfg.downgrade()),
    );

    let mut send_buf: [u8; 3] = [0; 3];
    send_buf[0] = READ_MASK | DEVID_REG;
    let reply = spi
        .transfer(&mut send_buf[0..1])
        .expect("Reading DEVID register failed");
    rprintln!("DEVID register: {}", reply[1]);
    loop {
        //let mut send_buf: [u8; 3] = [0x00, 0x01, 0x02];
        //spi.transfer(&mut send_buf[..]).unwrap();
        delay.delay_ms(500);
    }
}

#[interrupt]
fn OC0() {
    default_ms_irq_handler();
}
