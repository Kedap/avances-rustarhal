#![no_std]
#![no_main]

use arduino_hal::{
    delay_ms,
    port::{mode, Pin, PinOps},
    simple_pwm::{IntoPwmPin, Prescaler, Timer2Pwm},
};
use panic_halt as _;

struct Motor<T, U, V, W>
where
    T: PinOps,
    U: PinOps,
    V: arduino_hal::simple_pwm::PwmPinOps<W> + arduino_hal::port::PinOps,
{
    input1: Pin<mode::Output, T>,
    input2: Pin<mode::Output, U>,
    pwm: Pin<mode::PwmOutput<W>, V>,
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let timer3 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    let motor_in1 = pins.d9.into_output();
    let motor_in2 = pins.d10.into_output();
    let mut pwm_a = pins.d11.into_output().into_pwm(&timer3);
    pwm_a.enable();
    let mut stby = pins.d8.into_output();
    let mut motor1 = Motor {
        input1: motor_in1,
        input2: motor_in2,
        pwm: pwm_a,
    };

    stby.set_high();

    loop {
        control_motor(50, &mut motor1);
        control_motor(100, &mut motor1);
        control_motor(220, &mut motor1);
        control_motor(225, &mut motor1);
    }
}

fn control_motor<T, U, V, W>(variable_pwm: u8, motor: &mut Motor<T, U, V, W>)
where
    T: PinOps,
    U: PinOps,
    V: arduino_hal::simple_pwm::PwmPinOps<W> + arduino_hal::port::PinOps,
{
    for _ in 0..20 {
        motor.input1.set_low();
        motor.input2.set_high();
        motor.pwm.set_duty(variable_pwm);
        delay_ms(100);
    }

    for _ in 0..20 {
        motor.input1.set_low();
        motor.input2.set_low();
        motor.pwm.set_duty(variable_pwm);
        delay_ms(100);
    }

    for _ in 0..20 {
        motor.input1.set_high();
        motor.input2.set_low();
        motor.pwm.set_duty(variable_pwm);
        delay_ms(100);
    }

    for _ in 0..20 {
        motor.input1.set_low();
        motor.input2.set_low();
        motor.pwm.set_duty(variable_pwm);
        delay_ms(100);
    }
}
