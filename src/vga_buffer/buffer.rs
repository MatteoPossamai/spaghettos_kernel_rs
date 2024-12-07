use crate::vga_buffer::color::ColorCode;
use volatile::Volatile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub(crate) ascii_character: u8,
    pub(crate) color_code: ColorCode,
}

pub struct Buffer<const W: usize, const H: usize> {
    pub(crate) chars: [[Volatile<ScreenChar>; W]; H],
}
