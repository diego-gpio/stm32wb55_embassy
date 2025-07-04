use embassy_stm32::{
    Peri,
    exti::AnyChannel,
    gpio::{AnyPin, OutputType, Pin},
    lptim::pwm::Ch2,
    timer::{GeneralInstance4Channel, TimerChannel, TimerPin, simple_pwm::PwmPin},
};

#[embassy_executor::task]
pub async fn pwm(pin: Peri<'static, impl TimerPin>) {
    let pwm_pin = PwmPin::new(pin, OutputType::PushPull);
}
// async fn pwm_moisture(timer: Peri<'static, AnyPin> ,pin: Peri<'static, AnyPin>) {
// let pwm_pin = PwmPin::new(pin);
// let mut pwm = SimplePwm::new(timer, None, Some(pwm_pin), None, None, hz(2000), EdgeAlignedUp);
// let max_duty = pwm.get_max_duty();
// pwm.set_duty(Channel::Ch2, max_duty / 2);
// }
