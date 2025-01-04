use arduino_hal::{hal, DefaultClock};
use avr_device::interrupt;
use core::cell::RefCell;
use panic_halt as _;

pub type Console = hal::usart::Usart0<DefaultClock>;
pub static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

#[macro_export]
macro_rules! print {
    ($($t:tt)*) => {
        interrupt::free(|cs| {
            if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                let _ = ufmt::uwrite!(console, $($t)*);
            }
        })
    };
}

#[macro_export]
macro_rules! println {
    ($($t:tt)*) => {
        interrupt::free(|cs| {
            if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                let _ = ufmt::uwriteln!(console, $($t)*);
            }
        })
    };
}

pub fn put_console(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}

pub trait ToBin {
    fn to_bin(&self) -> [u8; 4];
}

impl ToBin for u8 {
    fn to_bin(&self) -> [u8; 4] {
        let mut bin_array = [0; 4];
        (0..4).for_each(|i| {
            bin_array[i] = (self >> i) & 1;
        });
        bin_array
    }
}
