use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use defmt::*;
use embassy_stm32::Peri;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::low_power::StopMode;
use embassy_stm32::low_power::stop_ready;
use embassy_time::Delay;
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn toggle_red(pin: Peri<'static, AnyPin>) {
    let mut led = Output::new(pin, Level::Low, Speed::Low);
    let mut delay = Delay;
    defmt::assert!(stop_ready(StopMode::Stop2));
    Timer::after_millis(500).await;
    loop {
        info!("Toggle LED 10 times");
        for _ in 1..10 {
            led.toggle();
            info!("Toggle RED !!");
            delay.delay_ms(100_u32);
        }
        led.set_low();
        if stop_ready(StopMode::Stop2) {
            info!("task led waiting for 5000ms...");
            Timer::after_millis(5000).await;
        }
    }
}

#[embassy_executor::task]
pub async fn toggle_green(pin: Peri<'static, AnyPin>) {
    let mut led = Output::new(pin, Level::Low, Speed::Low);
    let mut delay = Delay;
    defmt::assert!(stop_ready(StopMode::Stop2));
    Timer::after_millis(2500).await;
    loop {
        for _ in 0..10 {
            led.toggle();
            info!("Toggle BLUE !!");
            delay.delay_ms(100_u32);
        }
        led.set_low();
        if stop_ready(StopMode::Stop2) {
            info!("task led blue waiting for 5000ms...");
            Timer::after_millis(5000).await;
        }
    }
}
