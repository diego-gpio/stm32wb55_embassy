#![no_std]
#![no_main]

//use chrono::NaiveDate;
use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::low_power::Executor;
use embassy_stm32::rtc::{Rtc, RtcConfig};
use panic_probe as _;
use static_cell::StaticCell;

mod led;
// mod moisture;

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
    config.enable_debug_during_sleep = true;
    let p = embassy_stm32::init(config);

    let rtc = Rtc::new(p.RTC, RtcConfig::default());
    static RTC: StaticCell<Rtc> = StaticCell::new();
    let rtc = RTC.init(rtc);
    embassy_stm32::low_power::stop_with_rtc(rtc);

    spawner.spawn(led::toggle_red(p.PB1.into())).unwrap();
    spawner.spawn(led::toggle_green(p.PB0.into())).unwrap();
}
