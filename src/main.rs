// Anthony Wilcox licenses this file to you under the BSD 3-clause license.
// See the LICENSE file in the project root for more information.
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(dead_code)]

mod vga_buffer;
mod emulator;

use core::panic::PanicInfo;
use emulator::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to Glass.");
    loop {}
}
