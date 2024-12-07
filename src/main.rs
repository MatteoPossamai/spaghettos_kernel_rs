// main.rs
#![no_std]
#![no_main]

mod assetts;
mod handlers;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_start();

    loop {}
}
