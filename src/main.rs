#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_stm32::{
    exti::{AnyChannel, Channel, ExtiInput},
    gpio::{AnyPin, Pin, Pull},
};
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Starting...");
    let p = embassy_stm32::init(Default::default());
    let sw1 = button(p.PC4.degrade(), p.EXTI4.degrade(), "1");
    let sw3 = button(p.PD1.degrade(), p.EXTI1.degrade(), "1");
    join(sw1, sw3).await;
}

async fn button(pin: AnyPin, int: AnyChannel, id: &str) {
    let mut button = ExtiInput::new(pin, int, Pull::Up);
    loop {
        button.wait_for_falling_edge().await;
        info!("Button {} pressed!!", id);
        Timer::after_millis(200).await;
        button.wait_for_rising_edge().await;
    }
}
