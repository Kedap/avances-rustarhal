#![feature(asm_experimental_arch)]
#![no_std]
#![no_main]

use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};
const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const CPU_SPEED: u32 = 16_000_000;

#[no_mangle]
pub extern "C" fn main() {
    unsafe { write_volatile(DDRB, 0b11111111) };
    loop {
        let mut ports_value = unsafe { read_volatile(PORTB) };
        ports_value = ports_value ^ (1 << 5);
        sleep(1);
        unsafe { write_volatile(PORTB, ports_value) };
    }
}

fn sleep(time: u32) {
    for _ in 0..(CPU_SPEED / 10 * time) {
        unsafe { asm!("nop") }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
