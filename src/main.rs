#![no_std]
#![no_main]
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]
// #![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { //Implementing our own version of the panic handler.
	loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_buffer::write_something();
    loop {}
}