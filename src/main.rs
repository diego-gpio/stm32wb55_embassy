#![no_std]
#![no_main]
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::low_power::{Executor, StopMode, stop_ready, stop_with_rtc};
use embassy_stm32::rtc::{Rtc, RtcConfig};
use embassy_time::Timer;
use panic_probe as _;
use static_cell::StaticCell;

#[cortex_m_rt::entry]
fn main() -> ! {
    Executor::take().run(|spawner| {
        unwrap!(spawner.spawn(async_main(spawner)));
    });
}

#[embassy_executor::task]
async fn async_main(spawner: Spawner) {
    // initialize the platform...
    let mut config = embassy_stm32::Config::default();
    // when enabled the power-consumption is much higher during stop, but debugging and RTT is working
    config.enable_debug_during_sleep = false;
    static CONFIG: StaticCell<embassy_stm32::Config> = StaticCell::new();
    let config = CONFIG.init(config);
    let p = embassy_stm32::init(*config);

    // give the RTC to the executor...
    let rtc = Rtc::new(p.RTC, RtcConfig::default());
    static RTC: StaticCell<Rtc> = StaticCell::new();
    let rtc = RTC.init(rtc);
    stop_with_rtc(rtc);

    spawner.spawn(led_toggle(*config)).unwrap();
    spawner.spawn(task_2()).unwrap();
}

#[embassy_executor::task]
async fn led_toggle(config: embassy_stm32::Config) {
    let p = embassy_stm32::init(config);
    let mut led = Output::new(p.PB1, Level::Low, Speed::Low);
    for _ in 0..9 {
        info!("Toggle LED 10 times");
        led.toggle();
        Timer::after_millis(100).await;
    }
    // disable LED
    led.set_low();
    defmt::assert!(stop_ready(StopMode::Stop2));
}

#[embassy_executor::task]
async fn task_2() {
    for _ in 0..5 {
        info!("task 2: waiting for 1000ms...");
        defmt::assert!(stop_ready(StopMode::Stop2));
        Timer::after_millis(1000).await;
    }

    info!("Test OK");
    cortex_m::asm::bkpt();
}
