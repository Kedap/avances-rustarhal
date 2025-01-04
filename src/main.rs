#![no_std]
#![no_main]

mod comun;
mod sensores;

use crate::comun::{ToBin, CONSOLE};
use avr_device::interrupt;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 9600);
    comun::put_console(serial);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut led = pins.d11.into_output();
    let mut s0 = pins.a0.into_output();
    let mut s1 = pins.a1.into_output();
    let mut s2 = pins.a2.into_output();
    let mut s3 = pins.a3.into_output();
    led.set_high();
    let mut sensores = [0_u16; 16];
    println!("Setup completado {:#?}", 15.to_bin());

    loop {
        (0..16).for_each(|i| {
            let digital = (i as u8).to_bin();
            if digital[0] == 0 {
                s0.set_low();
            } else {
                s0.set_high();
            }
            if digital[1] == 0 {
                s1.set_low();
            } else {
                s1.set_high();
            }
            if digital[2] == 0 {
                s2.set_low();
            } else {
                s2.set_high();
            }
            if digital[3] == 0 {
                s3.set_low();
            } else {
                s3.set_high();
            }
            sensores[i] = adc.read_blocking(&arduino_hal::adc::channel::ADC6);
        });

        println!("Lectura: {:?}", sensores);
        arduino_hal::delay_ms(100);
    }
}
