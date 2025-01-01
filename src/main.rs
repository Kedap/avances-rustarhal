#![no_std]
#![no_main]

use arduino_hal::{hal, DefaultClock};
use avr_device::interrupt;
use core::cell::RefCell;
use panic_halt as _;

type Console = hal::usart::Usart0<DefaultClock>;
static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

macro_rules! print {
    ($($t:tt)*) => {
        interrupt::free(|cs| {
            if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                let _ = ufmt::uwrite!(console, $($t)*);
            }
        })
    };
}

macro_rules! println {
    ($($t:tt)*) => {
        interrupt::free(|cs| {
            if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                let _ = ufmt::uwriteln!(console, $($t)*);
            }
        })
    };
}

fn put_console(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}

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
    put_console(serial);

    println!("Hola desde main y Rust!");
    subfuncion();
    demo_print_sinln();

    loop {
        // NOP
    }
}
