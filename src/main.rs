#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut verde = pins.d3.into_output();
    let mut amarillo = pins.d4.into_output();
    let mut rojo = pins.d5.into_output();

    loop {
        verde.toggle();
        arduino_hal::delay_ms(1000);
        verde.toggle();
        amarillo.toggle();
        arduino_hal::delay_ms(1000);
        amarillo.toggle();
        rojo.toggle();
        arduino_hal::delay_ms(1000);
        rojo.toggle();
    }
}
