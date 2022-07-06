#![no_std]

use core::panic::PanicInfo;

// This Function is Called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {}
