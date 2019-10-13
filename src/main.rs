#![no_std]
#![no_main]
#![allow(dead_code)]

mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello world!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    vga_buffer::print_hello();

    loop {
    }
}