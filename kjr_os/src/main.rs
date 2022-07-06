#![no_std] // Don't Link Rust Standard Library
#![no_main] // Disable All Rust-Level Entry Points

use core::panic::PanicInfo;

// Through Implementing No Mangle, We Ensure the Rust Compiler
// Outputs a Function with the Name _start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Our Linked Looks for a Function Named _start by Default (Our Entrypoint)
    loop {}
}

// This Function is Called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
