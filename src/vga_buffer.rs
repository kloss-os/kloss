//! This module is an interface to the IBM standard VGA Buffer.

use core::ptr::Unique;
use core::fmt::Write;
use spin::Mutex;

// Proportions of the buffers.
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// An enum represented as an 8-bit integer mapping the various VGA BIOS
/// colours.
#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

/// Funny object: A `ColorCode` is just an 8-bit int.
#[derive(Clone, Copy)]
struct ColorCode(u8);

/// ...but notice how we are only implementing this library function for
/// `ColorCodes`, not u8 in general!
impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// Every character has a colour and an ascii character.
/// FIXME: We should really replace this with something
/// Unicode-compliant.
#[derive(Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl Writer {

    /// Write a given byte to the screen.
    /// Warning! Will only work with single bytes!
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer().chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };
                self.column_position += 1;
            }
        }
    }

    /// Write an entire string to screen, possibly clipping it if it
    /// turns out to be too long. Should not contain Unicode characters.
    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }

    /// Internal helper function. Get a mutable reference to the
    /// buffer. That `unsafe` block is sort of worrying.
    fn buffer(&mut self) -> &mut Buffer {
        unsafe{ self.buffer.get_mut() }
    }

    /// Print a simple newline.
    fn new_line(&mut self) {
        for row in 0..(BUFFER_HEIGHT-1) {
            let buffer = self.buffer();
            buffer.chars[row] = buffer.chars[row + 1]
        }
        self.clear_row(BUFFER_HEIGHT-1);
        self.column_position = 0;
    }

    /// Write an entire row of spaces, followed by a newline.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        self.buffer().chars[row] = [blank; BUFFER_WIDTH];

    }
}

/// Make our Writer implement the Writer trait. It just writes a given
/// set of bytes and returns Ok(()).
impl ::core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
          self.write_byte(byte)
        }
        Ok(())
    }
}

/// This looks kind of scary, but I think what it amounts to is a
/// spin-locked global static internal writer object, meaning that
/// anyone calling any of the writer macros below will automatically use
/// *this* writer, which is universally spin-locked for mutual exclusion.
pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Magenta, Color::White),
    buffer: unsafe { Unique::new(0xb8000 as *mut _) },
});

/// Implement the formatted println macro.
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
            use core::fmt::Write;
            $crate::vga_buffer::WRITER.lock().write_fmt(format_args!($($arg)*)).unwrap();
    });
}

/// Helper function: clear the screen by printing `BUFFER_HEIGHT` number
/// of newlines.
pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        println!("");
    }
}
