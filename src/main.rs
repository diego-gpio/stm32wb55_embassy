#![no_std]
#![no_main]

use cortex_m as _;
use cortex_m::delay::Delay;
use cortex_m_rt::entry; // The runtime

use defmt::{info, warn};
use defmt_rtt as _;
use hal::{
    self,
    clocks::Clocks,
    gpio::{Pin, PinMode, Port, Pull},
    pac,
};
use panic_halt as _;

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
    // Set up CPU peripherals
    info!("Starting...");
    let cp = cortex_m::Peripherals::take().unwrap();
    // Set up microcontroller peripherals
    let mut _dp = pac::Peripherals::take().unwrap();
    warn!("I'm here");

    let clock_cfg = Clocks::default();

    // Write the clock configuration to the MCU. If you wish, you can modify `clocks` above
    // in accordance with [its docs](https://docs.rs/stm32-hal2/0.2.0/stm32_hal2/clocks/index.html),
    // and the `clock_cfg` example.
    clock_cfg.setup().unwrap();

    // Setup a delay, based on the Cortex-m systick.
    info!("Configuring peripherals");
    let mut delay = Delay::new(cp.SYST, clock_cfg.systick());
    let mut led1 = Pin::new(Port::B, 5, PinMode::Output);
    let mut led2 = Pin::new(Port::B, 0, PinMode::Output);
    let mut led3 = Pin::new(Port::B, 1, PinMode::Output);
    let mut sw1 = Pin::new(Port::C, 4, PinMode::Input);
    let mut sw2 = Pin::new(Port::D, 0, PinMode::Input);
    sw1.pull(Pull::Up);
    sw2.pull(Pull::Up);
    // let mut sw3 = Pin::new(Port::D, 1, PinMode::Input);

    loop {
        if sw1.is_low() {
            info!("Turning ON LEDs");
            led1.set_high();
            led2.set_high();
            led3.set_high();
            delay.delay_ms(1_000);
        }
        if sw2.is_low() {
            info!("Turning OFF LEDs");
            led1.set_low();
            led2.set_low();
            led3.set_low();
            delay.delay_ms(1_000);
        }
    }
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
