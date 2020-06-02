use core::{convert::TryFrom, fmt};

use lazy_static::lazy_static;

use spin::Mutex;

use volatile::Volatile;

/// A color that can be used in VGA text mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
}

impl Into<u8> for Color {
    fn into(self) -> u8 {
        match self {
            Self::Black => 0x00,
            Self::Blue => 0x01,
            Self::Green => 0x02,
            Self::Cyan => 0x03,
            Self::Red => 0x04,
            Self::Magenta => 0x05,
            Self::Brown => 0x06,
            Self::LightGray => 0x07,
        }
    }
}

impl TryFrom<u8> for Color {
    type Error = u8;

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0x00 => Ok(Self::Black),
            0x01 => Ok(Self::Blue),
            0x02 => Ok(Self::Green),
            0x03 => Ok(Self::Cyan),
            0x04 => Ok(Self::Red),
            0x05 => Ok(Self::Magenta),
            0x06 => Ok(Self::Brown),
            0x07 => Ok(Self::LightGray),
            0x08..=0xFF => Err(b),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackgroundColor {
    NoBlink(Color),
    Blink(Color),
}

impl BackgroundColor {
    pub const SOLID_BLACK: Self = Self::NoBlink(Color::Black);
    pub const SOLID_BLUE: Self = Self::NoBlink(Color::Blue);
    pub const SOLID_GREEN: Self = Self::NoBlink(Color::Green);
    pub const SOLID_CYAN: Self = Self::NoBlink(Color::Cyan);
    pub const SOLID_RED: Self = Self::NoBlink(Color::Red);
    pub const SOLID_MAGENTA: Self = Self::NoBlink(Color::Magenta);
    pub const SOLID_BROWN: Self = Self::NoBlink(Color::Brown);
    pub const SOLID_LIGHT_GRAY: Self = Self::NoBlink(Color::LightGray);
    pub const BLINK_BLACK: Self = Self::Blink(Color::Black);
    pub const BLINK_BLUE: Self = Self::Blink(Color::Blue);
    pub const BLINK_GREEN: Self = Self::Blink(Color::Green);
    pub const BLINK_CYAN: Self = Self::Blink(Color::Cyan);
    pub const BLINK_RED: Self = Self::Blink(Color::Red);
    pub const BLINK_MAGENTA: Self = Self::Blink(Color::Magenta);
    pub const BLINK_BROWN: Self = Self::Blink(Color::Brown);
    pub const BLINK_LIGHT_GRAY: Self = Self::Blink(Color::LightGray);
}

impl Into<u8> for BackgroundColor {
    fn into(self) -> u8 {
        match self {
            Self::NoBlink(c) => c as u8,
            Self::Blink(c) => 0x08 | c as u8,
        }
    }
}

impl TryFrom<u8> for BackgroundColor {
    type Error = u8;

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0x00..=0x07 => Color::try_from(b).map(|c| Self::NoBlink(c)),
            0x08..=0x0F => Color::try_from(b & 0x07).map(|c| Self::Blink(c)),
            0x10..=0xFF => Err(b),
        }
    }
}

/// A color that can be used for text in VGA text mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextColor {
    Dark(Color),
    Light(Color),
}

impl TextColor {
    pub const BLACK: Self = Self::Dark(Color::Black);
    pub const BLUE: Self = Self::Dark(Color::Blue);
    pub const GREEN: Self = Self::Dark(Color::Green);
    pub const CYAN: Self = Self::Dark(Color::Cyan);
    pub const RED: Self = Self::Dark(Color::Red);
    pub const MAGENTA: Self = Self::Dark(Color::Magenta);
    pub const BROWN: Self = Self::Dark(Color::Brown);
    pub const LIGHT_GRAY: Self = Self::Dark(Color::LightGray);
    pub const DARK_GRAY: Self = Self::Light(Color::Black);
    pub const LIGHT_BLUE: Self = Self::Light(Color::Blue);
    pub const LIGHT_GREEN: Self = Self::Light(Color::Green);
    pub const LIGHT_CYAN: Self = Self::Light(Color::Cyan);
    pub const LIGHT_RED: Self = Self::Light(Color::Red);
    pub const PINK: Self = Self::Light(Color::Magenta);
    pub const YELLOW: Self = Self::Light(Color::Brown);
    pub const WHITE: Self = Self::Light(Color::LightGray);
}

impl Into<u8> for TextColor {
    fn into(self) -> u8 {
        match self {
            Self::Dark(c) => c as u8,
            Self::Light(c) => 0x08 | c as u8,
        }
    }
}

impl TryFrom<u8> for TextColor {
    type Error = u8;

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0x00..=0x07 => Color::try_from(b).map(|c| Self::Dark(c)),
            0x08..=0x0F => Color::try_from(b & 0x07).map(|c| Self::Light(c)),
            0x10..=0xFF => Err(b),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct CharColor(pub u8);

impl From<(BackgroundColor, TextColor)> for CharColor {
    fn from((bg_color, text_color): (BackgroundColor, TextColor)) -> Self {
        Self((<BackgroundColor as Into<u8>>::into(bg_color)) << 4 | <TextColor as Into<u8>>::into(text_color))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
struct ScreenChar {
    c: u8,
    color: CharColor,
}

struct Buffer {
    chars: [[Volatile<ScreenChar>; Self::CHARS_PER_LINE]; Self::HEIGHT],
}

impl Buffer {
    const CHARS_PER_LINE: usize = 80;
    const HEIGHT: usize = 24;
}

pub struct Writer {
    column: usize,
    color: CharColor,
    buffer: &'static mut Buffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column: 0,
        color: CharColor(0x0b),
        buffer: unsafe { (0xb_8000 as *mut Buffer).as_mut().unwrap() },
    });
}

impl Writer {
    pub fn crlf(&mut self) {
        for line in 1..Buffer::HEIGHT {
            for col in 0..Buffer::CHARS_PER_LINE {
                self.buffer.chars[line - 1][col].write(self.buffer.chars[line][col].read());
            }
        }
        for col in 0..Buffer::CHARS_PER_LINE {
            self.buffer.chars[Buffer::HEIGHT - 1][col].write(ScreenChar {
                c: 0,
                color: CharColor(0x00),
            });
        }
        self.column = 0;
    }

    pub fn set_color(&mut self, color: CharColor) {
        self.color = color;
    }

    pub fn write<Bytes>(&mut self, bytes: Bytes)
    where
      Bytes: IntoIterator<Item = u8>,
    {
        bytes.into_iter().for_each(|byte| self.write_byte(byte));
    }

    pub fn write_byte(&mut self, byte: u8) {
        let byte = match byte {
            b'\n' => return self.crlf(),
            0x00..=0x7F => byte,
            0x80..=0xFF => 0xEF,
        };
        self.buffer.chars[Buffer::HEIGHT - 1][self.column].write(ScreenChar {
            c: byte,
            color: self.color,
        });
        self.column += 1;
        if self.column >= Buffer::CHARS_PER_LINE {
            self.crlf();
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s.bytes());
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    super::print_to(&mut *WRITER.lock(), args, "VGA port");
}

#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => ($crate::io::vga_text::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! vga_println {
    () => ($crate::vga_print!("\n"));
    ($($arg:tt)*) => ($crate::vga_print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! set_vga_color {
    ($color:expr) => {
        $crate::io::vga_text::WRITER
            .lock()
            .set_color($crate::io::vga_text::CharColor::from($color))
    };
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::{print, println};

    const TEST_PREFIX: &'static str = "[rust_os::io::vga_text]";

    #[test_case]
    fn test_color_from_0() {
        print!("{} test_color_from_0... ", TEST_PREFIX);
        assert!(Color::try_from(0x00).ok().is_some());
        println!("[ok]");
    }

    #[test_case]
    fn test_color_from_7() {
        print!("{} test_color_from_7... ", TEST_PREFIX);
        assert!(Color::try_from(0x07).ok().is_some());
        println!("[ok]");
    }

    #[test_case]
    fn test_color_from_8() {
        print!("{} test_color_from_8... ", TEST_PREFIX);
        assert!(Color::try_from(0x08).ok().is_none());
        println!("[ok]");
    }

    #[test_case]
    fn test_vga_println_succeeds() {
        print!("{} test_vga_println_succeeds... ", TEST_PREFIX);
        vga_println!("Testing VGA println");
        println!("[ok]");
    }

    #[test_case]
    fn test_vga_println_many() {
        print!("{} test_vga_println_many... ", TEST_PREFIX);
        for _ in 0..200 {
            vga_println!();
        }
        println!("[ok]");
    }

    #[test_case]
    fn test_println_output() {
        print!("{} test_println_output... ", TEST_PREFIX);
        let s = "Some test string that fits on a single line";
        vga_println!("{}", s);
        for (i, b) in s.bytes().enumerate() {
            let screen_byte = WRITER.lock().buffer.chars[Buffer::HEIGHT - 2][i].read();
            assert_eq!(screen_byte.c, b);
        }
        println!("[ok]");
    }
}
