#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!"; //We don't have access to any String::from functions so we
                                       //must create an array of characters on our own.

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { //Implementing our own version of the panic handler.
	loop {}
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; //EACH INCREMENT, WE MOVE 2 BYTES
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
