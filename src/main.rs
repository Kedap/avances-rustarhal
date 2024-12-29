#![no_std]
#![no_main]

use arduino_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer1Pwm};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    let mut rojo = pins.d9.into_output().into_pwm(&timer1);
    rojo.enable();

    loop {
        for x in (0..255).chain((0..255).rev()) {
            rojo.set_duty(x);
            arduino_hal::delay_ms(1000);
        }
    }
}
