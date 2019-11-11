// Anthony Wilcox licenses this file to you under the BSD 3-clause license.
// See the LICENSE file in the project root for more information.
#![no_std]
#![no_main]
#![allow(dead_code)]

mod emulator;
mod serial;
mod vga_buffer;

use core::panic::PanicInfo;
use emulator::*;
use test::test_main;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to Glass.");

    #[cfg(test)]
    test_main();
    loop {}
}
