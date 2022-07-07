#![no_std] // Don't Link Rust Standard Library
#![no_main] // Disable All Rust-Level Entry Points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";
// Through Implementing No Mangle, We Ensure the Rust Compiler
// Outputs a Function with the Name _start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// This Function is Called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
