// main.rs
#![no_std]
#![no_main]

mod handlers;
mod vga_buffer;
mod assetts;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    unsafe {
        vga_buffer::print_start();
    }

    loop {}
}
