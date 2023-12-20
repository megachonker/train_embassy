#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pin, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn run_blink(led: AnyPin) {
    let mut led = Output::new(led, Level::High, Speed::Low);
    loop {
        led.toggle();
        Timer::after_millis(100).await;
    }
}

#[embassy_executor::task]
async fn mesure(mesure: AnyPin, power: AnyPin) {
    //A0 => PA3
    let mut a0 = Input::new(mesure, embassy_stm32::gpio::Pull::Down);
    let _pd7 = Output::new(power, Level::High, Speed::Low);

    loop {
        info!("{}", a0.is_high());
        Timer::after_millis(300).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    spawner.spawn(run_blink(p.PB7.degrade())).unwrap();
    spawner
        .spawn(mesure(p.PA3.degrade(), p.PD7.degrade()))
        .unwrap();
}
