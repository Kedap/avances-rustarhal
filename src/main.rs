#![no_std]
#![no_main]

mod comun;
mod sensores;

use crate::comun::CONSOLE;
use crate::sensores::Sensores;
use avr_device::interrupt;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 9600);
    comun::put_console(serial);

    let mut sensores = sensores_default!(dp, pins);

    loop {
        sensores.actualizar_valores();
        println!("Lectura: {:?}", sensores);
        arduino_hal::delay_ms(100);
    }
}
