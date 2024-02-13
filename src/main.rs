//cargo flash --chip stm32f411ceux --release

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;
use multi_mission_library::usb::USB;

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();

    let gpioa = p.GPIOA.split();
    let tx_pin = gpioa.pa9;
    let rx_pin = gpioa.pa10;

    let mut usb = USB::new(p.USART1, rx_pin, tx_pin, &clocks);

    loop {
        usb.println("Hello, World!");
    }
}