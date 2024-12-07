mod buffer;
mod color;
mod writer;

use color::{Color, ColorCode};

use crate::{println, set_color_code};

pub fn print_start() {
    println!("HELLO");
    let color_code = ColorCode::new(Color::Magenta, Color::White);
    set_color_code!(color_code);
    println!("COME VAL")
}
