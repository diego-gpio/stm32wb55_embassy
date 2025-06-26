#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_stm32::{
    exti::{AnyChannel, Channel, ExtiInput},
    gpio::{AnyPin, Level, Output, Pin, Pull, Speed},
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, signal::Signal};
use embassy_time::{Duration, Timer, WithTimeout};
use panic_probe as _;

#[derive(Clone, Copy)]
enum Button {
    SW1,
    SW3,
}

static SIGNAL: Signal<ThreadModeRawMutex, Button> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting...");
    let p = embassy_stm32::init(Default::default());
    spawner.spawn(led_toggle(p.PB1.degrade())).unwrap();
    let sw1 = button(p.PC4.degrade(), p.EXTI4.degrade(), "1", Button::SW1);
    let sw3 = button(p.PD1.degrade(), p.EXTI1.degrade(), "3", Button::SW3);
    join(sw1, sw3).await;
}

async fn button(pin: AnyPin, int: AnyChannel, id: &str, b: Button) {
    let mut button = ExtiInput::new(pin, int, Pull::Up);
    loop {
        button.wait_for_falling_edge().await;
        info!("Button {} pressed!!", id);
        SIGNAL.signal(b);
        Timer::after_millis(200).await;
        button.wait_for_rising_edge().await;
    }
}

#[embassy_executor::task]
async fn led_toggle(pin: AnyPin) {
    const INTERVAL_MS: u64 = 500;
    let mut delay_ms: u64 = INTERVAL_MS;
    let mut led = Output::new(pin, Level::Low, Speed::Low);
    loop {
        led.toggle();
        let delay: Duration = Duration::from_millis(delay_ms);
        if let Some(v) = SIGNAL.wait().with_timeout(delay).await.ok() {
            delay_ms = match v {
                Button::SW1 if delay_ms > INTERVAL_MS => delay_ms - INTERVAL_MS,
                Button::SW1 => delay_ms,
                Button::SW3 => delay_ms + INTERVAL_MS,
            };
            info!("Delay = {} ms", delay_ms);
        }
    }
}
