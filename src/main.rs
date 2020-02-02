// Anthony Foxclaw licenses this file to you under the MPL-2.0 license.
// See the LICENSE file in the project root for more information.
#![no_std]
#![no_main]
#![allow(dead_code)]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

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
