#![no_std] // Disable the standard library
#![no_main] // Prevent using Rust's default entry point

mod assetts;
mod print_utils;

use core::panic::PanicInfo;

/// This function is called if the kernel panics.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // The VGA buffer should be accessed as a mutable pointer
    let vga_buffer = 0xb8000 as *mut u8;

    unsafe {
        // Write the character
        *vga_buffer = b'K';
        // Write the color attribute
        *vga_buffer.offset(1) = 0x07;
    }

    loop {}
}