#![no_std]
#![no_main]

mod serial;

use crate::serial::CONSOLE;
use avr_device::interrupt;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 9600);
    serial::put_console(serial);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut led = pins.d11.into_output();
    let mut s0 = pins.a0.into_output();
    let mut s1 = pins.a1.into_output();
    let mut s2 = pins.a2.into_output();
    let mut s3 = pins.a3.into_output();
    println!("Setup completado");

    loop {
        led.set_high();
        s0.set_low();
        s1.set_low();
        s2.set_low();
        s3.set_low();

        println!(
            "Lectura: {}",
            adc.read_blocking(&arduino_hal::adc::channel::ADC6)
        );
        arduino_hal::delay_ms(100);
    }
}
