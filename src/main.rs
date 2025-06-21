#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use cortex_m_rt::entry; // The runtime
use hal::{
    self,
    clocks::Clocks,
    gpio::{Pin, PinMode, Port},
    pac,
};
use panic_halt as _;

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
    // Set up CPU peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    // Set up microcontroller peripherals
    let mut _dp = pac::Peripherals::take().unwrap();

    let clock_cfg = Clocks::default();

    // Write the clock configuration to the MCU. If you wish, you can modify `clocks` above
    // in accordance with [its docs](https://docs.rs/stm32-hal2/0.2.0/stm32_hal2/clocks/index.html),
    // and the `clock_cfg` example.
    clock_cfg.setup().unwrap();

    // Setup a delay, based on the Cortex-m systick.
    let mut delay = Delay::new(cp.SYST, clock_cfg.systick());
    // Port::C, 13 because the LED is described as PC13 on WeAct blackpill page
    let mut led1 = Pin::new(Port::B, 5, PinMode::Output);
    let mut led2 = Pin::new(Port::B, 0, PinMode::Output);
    let mut led3 = Pin::new(Port::B, 1, PinMode::Output);

    loop {
        led1.set_low();
        delay.delay_ms(1_000);
        led1.set_high();
        delay.delay_ms(1_000);
        led2.set_low();
        delay.delay_ms(1_000);
        led2.set_high();
        delay.delay_ms(1_000);
        led3.set_low();
        delay.delay_ms(1_000);
        led3.set_high();
        delay.delay_ms(1_000);
    }
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
