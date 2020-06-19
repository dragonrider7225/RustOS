use core::{
    convert::TryFrom,
    fmt::{self, Arguments, Write},
};

use lazy_static::lazy_static;

use spin::Mutex;

use volatile::Volatile;

/// The base for two colors that can be used in CGA text mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum CgaColor {
    /// A black or dark gray color. The RGB for the dark form of this color is `#000000`. The RGB
    /// for the light form of this color is `#555555`.
    Black,
    /// A blue color. The RGB for the dark form of this color is `#0000AA`. The RGB for the light
    /// form of this color is `#5555FF`.
    Blue,
    /// A green color. The RGB for the dark form of this color is `#00AA00`. The RGB for the light
    /// form of this color is `#55FF55`.
    Green,
    /// A cyan color. The RGB for the dark form of this color is `#00AAAA`. The RGB for the light
    /// form of this color is `#55FFFF`.
    Cyan,
    /// A red color. The RGB for the dark form of this color is `#AA0000`. The RGB for the light
    /// form of this color is `#FF5555`.
    Red,
    /// A magenta or pink color. The RGB for the dark form of this color is `#AA00AA`. The RGB for
    /// the light form of this color is `#FF55FF`.
    Magenta,
    /// A brown or yellow color. The RGB for the dark form of this color is `#AA5500`. The RGB for
    /// the light form of this color is `#FFFF55`.
    Brown,
    /// A light gray or white color. The RGB for the dark form of this color is `#AAAAAA`. The RGB
    /// for the light form of this color is `#FFFFFF`.
    LightGray,
}

impl CgaColor {
    /// Get an iterator over the available base colors.
    pub fn colors() -> impl Iterator<Item = Self> {
        [
            Self::Black,
            Self::Blue,
            Self::Green,
            Self::Cyan,
            Self::Red,
            Self::Magenta,
            Self::Brown,
            Self::LightGray,
        ]
        .iter()
        .copied()
    }
}

impl Into<u8> for CgaColor {
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

impl TryFrom<u8> for CgaColor {
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

/// A color that can be used for the background in VGA text mode. Each of the eight base colors can
/// be used as the background color and each color can optionally make text on it blink. Some
/// implementations of VGA text use the light form of the base color instead of making the text
/// blink.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackgroundColor {
    /// Make the background color `self.0`.
    NoBlink(CgaColor),
    /// Make the background color `self.0` and make the text blink. Some implementations of VGA
    /// text instead make the background color the light form of `self.0` and don't make the text
    /// blink.
    Blink(CgaColor),
}

impl BackgroundColor {
    /// The non-blink form of `Color::Black`.
    pub const SOLID_BLACK: Self = Self::NoBlink(CgaColor::Black);
    /// The non-blink form of `Color::Blue`.
    pub const SOLID_BLUE: Self = Self::NoBlink(CgaColor::Blue);
    /// The non-blink form of `Color::Green`.
    pub const SOLID_GREEN: Self = Self::NoBlink(CgaColor::Green);
    /// The non-blink form of `Color::Cyan`.
    pub const SOLID_CYAN: Self = Self::NoBlink(CgaColor::Cyan);
    /// The non-blink form of `Color::Red`.
    pub const SOLID_RED: Self = Self::NoBlink(CgaColor::Red);
    /// The non-blink form of `Color::Magenta`.
    pub const SOLID_MAGENTA: Self = Self::NoBlink(CgaColor::Magenta);
    /// The non-blink form of `Color::Brown`.
    pub const SOLID_BROWN: Self = Self::NoBlink(CgaColor::Brown);
    /// The non-blink form of `Color::LightGray`.
    pub const SOLID_LIGHT_GRAY: Self = Self::NoBlink(CgaColor::LightGray);
    /// The blink form of `Color::Black`.
    pub const BLINK_BLACK: Self = Self::Blink(CgaColor::Black);
    /// The blink form of `Color::Blue`.
    pub const BLINK_BLUE: Self = Self::Blink(CgaColor::Blue);
    /// The blink form of `Color::Green`.
    pub const BLINK_GREEN: Self = Self::Blink(CgaColor::Green);
    /// The blink form of `Color::Cyan`.
    pub const BLINK_CYAN: Self = Self::Blink(CgaColor::Cyan);
    /// The blink form of `Color::Red`.
    pub const BLINK_RED: Self = Self::Blink(CgaColor::Red);
    /// The blink form of `Color::Magenta`.
    pub const BLINK_MAGENTA: Self = Self::Blink(CgaColor::Magenta);
    /// The blink form of `Color::Brown`.
    pub const BLINK_BROWN: Self = Self::Blink(CgaColor::Brown);
    /// The blink form of `Color::LightGray`.
    pub const BLINK_LIGHT_GRAY: Self = Self::Blink(CgaColor::LightGray);

    /// Get an iterator over the background colors.
    pub fn colors() -> impl Iterator<Item = Self> {
        [
            Self::SOLID_BLACK,
            Self::SOLID_BLUE,
            Self::SOLID_GREEN,
            Self::SOLID_CYAN,
            Self::SOLID_RED,
            Self::SOLID_MAGENTA,
            Self::SOLID_BROWN,
            Self::SOLID_LIGHT_GRAY,
            Self::BLINK_BLACK,
            Self::BLINK_BLUE,
            Self::BLINK_GREEN,
            Self::BLINK_CYAN,
            Self::BLINK_RED,
            Self::BLINK_MAGENTA,
            Self::BLINK_BROWN,
            Self::BLINK_LIGHT_GRAY,
        ]
        .iter()
        .copied()
    }
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
            0x00..=0x07 => CgaColor::try_from(b).map(|c| Self::NoBlink(c)),
            0x08..=0x0F => CgaColor::try_from(b & 0x07).map(|c| Self::Blink(c)),
            0x10..=0xFF => Err(b),
        }
    }
}

/// A color that can be used for text in VGA text mode. Each of the eight base colors can be either
/// dark or light when applied to text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextColor {
    /// Make the text color the dark form of `self.0`.
    Dark(CgaColor),
    /// Make the text color the light form of `self.0`.
    Light(CgaColor),
}

impl TextColor {
    /// The dark form of `Color::Black`.
    pub const BLACK: Self = Self::Dark(CgaColor::Black);
    /// The dark form of `Color::Blue`.
    pub const BLUE: Self = Self::Dark(CgaColor::Blue);
    /// The dark form of `Color::Green`.
    pub const GREEN: Self = Self::Dark(CgaColor::Green);
    /// The dark form of `Color::Cyan`.
    pub const CYAN: Self = Self::Dark(CgaColor::Cyan);
    /// The dark form of `Color::Red`.
    pub const RED: Self = Self::Dark(CgaColor::Red);
    /// The dark form of `Color::Magenta`.
    pub const MAGENTA: Self = Self::Dark(CgaColor::Magenta);
    /// The dark form of `Color::Brown`.
    pub const BROWN: Self = Self::Dark(CgaColor::Brown);
    /// The dark form of `Color::LightGray`.
    pub const LIGHT_GRAY: Self = Self::Dark(CgaColor::LightGray);
    /// The light form of `Color::Black`.
    pub const DARK_GRAY: Self = Self::Light(CgaColor::Black);
    /// The light form of `Color::Blue`.
    pub const LIGHT_BLUE: Self = Self::Light(CgaColor::Blue);
    /// The light form of `Color::Green`.
    pub const LIGHT_GREEN: Self = Self::Light(CgaColor::Green);
    /// The light form of `Color::Cyan`.
    pub const LIGHT_CYAN: Self = Self::Light(CgaColor::Cyan);
    /// The light form of `Color::Red`.
    pub const LIGHT_RED: Self = Self::Light(CgaColor::Red);
    /// The light form of `Color::Magenta`.
    pub const PINK: Self = Self::Light(CgaColor::Magenta);
    /// The light form of `Color::Brown`.
    pub const YELLOW: Self = Self::Light(CgaColor::Brown);
    /// The light form of `Color::LightGray`.
    pub const WHITE: Self = Self::Light(CgaColor::LightGray);

    /// Get an iterator over the text colors.
    pub fn colors() -> impl Iterator<Item = Self> {
        [
            Self::BLACK,
            Self::BLUE,
            Self::GREEN,
            Self::CYAN,
            Self::RED,
            Self::MAGENTA,
            Self::BROWN,
            Self::LIGHT_GRAY,
            Self::DARK_GRAY,
            Self::LIGHT_BLUE,
            Self::LIGHT_GREEN,
            Self::LIGHT_CYAN,
            Self::LIGHT_RED,
            Self::PINK,
            Self::YELLOW,
            Self::WHITE,
        ]
        .iter()
        .copied()
    }
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
            0x00..=0x07 => CgaColor::try_from(b).map(|c| Self::Dark(c)),
            0x08..=0x0F => CgaColor::try_from(b & 0x07).map(|c| Self::Light(c)),
            0x10..=0xFF => Err(b),
        }
    }
}

/// A combination text color and character color.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct CharColor(pub u8);

impl From<(BackgroundColor, TextColor)> for CharColor {
    fn from((bg_color, text_color): (BackgroundColor, TextColor)) -> Self {
        Self(
            (<BackgroundColor as Into<u8>>::into(bg_color)) << 4
                | <TextColor as Into<u8>>::into(text_color),
        )
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

/// A writer to a VGA-like output buffer.
pub struct Writer {
    column: usize,
    color: CharColor,
    buffer: &'static mut Buffer,
}

lazy_static! {
    /// The singleton `Writer` instance.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column: 0,
        color: CharColor::from(Writer::DEFAULT_COLOR_PAIR),
        buffer: unsafe { (0xb_8000 as *mut Buffer).as_mut().unwrap() },
    });
}

impl Writer {
    /// The initial color for the `Writer`.
    pub const DEFAULT_COLOR_PAIR: (BackgroundColor, TextColor) =
        (BackgroundColor::SOLID_BLACK, TextColor::LIGHT_GREEN);

    /// Start a new line in the `Writer`.
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

    /// Set the color for all new characters written to the `Writer`.
    pub fn set_color(&mut self, color: CharColor) {
        self.color = color;
    }

    /// Write the bytestring `bytes` to the `Writer` in the current color.
    pub fn write<Bytes>(&mut self, bytes: Bytes)
    where
        Bytes: IntoIterator<Item = u8>,
    {
        bytes.into_iter().for_each(|byte| self.write_byte(byte));
    }

    /// Write the byte `byte` to the `Writer` in the current color.
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

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s.bytes());
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    super::print_to(&mut *WRITER.lock(), args, "VGA port");
}

/// Print a formatted string to the VGA text buffer with the current color.
#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => ($crate::io::vga_text::_print(format_args!($($arg)*)));
}

/// Print a formatted string to the VGA text buffer with the current color. Terminate with a
/// newline.
#[macro_export]
macro_rules! vga_println {
    () => ($crate::vga_print!("\n"));
    ($($arg:tt)*) => ($crate::vga_print!("{}\n", format_args!($($arg)*)));
}

/// Set the color to use for the VGA text buffer until a different color is selected.
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
        assert!(CgaColor::try_from(0x00).ok().is_some());
        println!("[ok]");
    }

    #[test_case]
    fn test_color_from_7() {
        print!("{} test_color_from_7... ", TEST_PREFIX);
        assert!(CgaColor::try_from(0x07).ok().is_some());
        println!("[ok]");
    }

    #[test_case]
    fn test_color_from_8() {
        print!("{} test_color_from_8... ", TEST_PREFIX);
        assert!(CgaColor::try_from(0x08).ok().is_none());
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
