#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(non_snake_case)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] 
pub extern "C" fn _start() -> ! {
    loop {}
}