#![no_std] // Disable the standard library
#![no_main] // Prevent using Rust's default entry point

mod assetts;
mod print_utils;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // let vga_buffer: *mut u8 = 0xb8000 as *mut u8;
    // unsafe {
    //     *vga_buffer = b'K';
    //     *(vga_buffer.add(1)) = 0x07;
    // }
    unsafe {
        print_utils::print_start();
    }
    loop {}
}
