#![no_std]
#![no_main]

mod serial;

use crate::serial::CONSOLE;
use avr_device::interrupt;
use panic_halt as _;

fn subfuncion() {
    println!("Podemos usar el println en serial");
}

fn demo_print_sinln() {
    for i in 0..10 {
        print!("{} ", i);
    }
    println!("Numeros");
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 9600);
    serial::put_console(serial);

    println!("Hola desde main y Rust modificado!");
    subfuncion();
    demo_print_sinln();

    loop {
        // NOP
    }
}
