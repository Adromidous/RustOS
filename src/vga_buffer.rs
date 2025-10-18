#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] //Store as 8 bits, u4 doesn't exist
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] //Used on structs/enums with one field.
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground_color: Color, background_color: Color) -> ColorCode {
        ColorCode(((background_color as u8) << 4) | (foreground_color as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

pub fn write_char(current_char: u8) {

    //Case 1: current_char == '\n'

    //Case 2: 0 <= column_position <= BUFFER_WIDTH

}