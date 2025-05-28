#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { //Implementing our own version of the panic handler.
	loop {}
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
	loop {}
}
