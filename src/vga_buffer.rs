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

use volatile::Volatile;

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
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {

    fn write_char(&mut self, curr_char: u8) {

        match curr_char {
            b'\n' => self.new_line(),
            _ => {
                if self.column_position == BUFFER_WIDTH - 1 {
                    self.column_position = 0;
                }

                let row = BUFFER_HEIGHT-1;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar { 
                    ascii_character: (curr_char), 
                    color_code: (self.color_code)
                });

                self.column_position += 1;
            },
        }

    }

    fn new_line(&mut self) {
        for row in 0..BUFFER_HEIGHT-1 {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row][col].write(self.buffer.chars[row+1][col].read());
            }
        }

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[BUFFER_HEIGHT-1][col].write(ScreenChar { 
                ascii_character: (0x0), 
                color_code: (self.color_code) 
            });
        }

        self.column_position = 0;
    }

    pub fn write_string(&mut self, curr_str: &str) {
        for byte in curr_str.bytes() {
            match byte {
                0x20..=0x70 | b'\n' => {self.write_char(byte)},
                _=> {self.write_char(0xfe)},
            }
        }
    }
}

use core::fmt::{self, Error};

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.write_string(s);
        Ok(())
    }
}

pub fn write_something() {
    let mut writer: Writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
    };

    writer.write_string("Hello World\n");
    writer.write_string("Adromidous!");
    writer.write_string("\n");
    writer.write_string("\n");
    writer.write_string("\n");
}