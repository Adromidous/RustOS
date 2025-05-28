#![no_std]

fn main() {}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { //Implementing our own version of the panic handler.
	loop {}
}
