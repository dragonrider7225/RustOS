use core::fmt::{Arguments, Write};

use spin::MutexGuard;

/// Various tools for writing to the serial port.
pub mod serial;

/// Various tools for writing in VGA text mode.
pub mod vga_text;

#[cfg(not(test))]
mod _impl {
    use super::*;
    use vga_text::Writer;

    /// Get exclusive access to `stdout`.
    pub fn stdout<'a>() -> MutexGuard<'a, Writer> {
        vga_text::WRITER.lock()
    }

    /// Set the color of `stdout`. Once `stdout`'s color has been set, it will remain that color
    /// until it is set again.
    #[macro_export]
    macro_rules! set_stdout_color {
        ($color:expr) => {
            $crate::io::stdout().set_color($crate::io::vga_text::CharColor::from($color))
        };
    }
}

#[cfg(test)]
mod _impl {
    use super::*;
    use uart_16550::SerialPort;

    /// Get exclusive access to `stdout`.
    pub fn stdout<'a>() -> MutexGuard<'a, SerialPort> {
        serial::SERIAL1.lock()
    }

    /// Set the color of `stdout`. Once `stdout`'s color has been set, it will remain that color
    /// until it is set again. As serial out does not support coloration, the argument is ignored.
    #[macro_export]
    macro_rules! set_stdout_color {
        ($color:expr) => {{
            let _ignore = $color;
        }};
    }
}

pub use _impl::stdout;

/// Write a formatted string to an output stream named `name`.
#[doc(hidden)]
pub fn print_to(out: &mut dyn Write, args: Arguments, name: &str) {
    out.write_fmt(args)
        .unwrap_or_else(|_| panic!("Failed to write to {}: {}", name, args));
}

/// Write a formatted string to stdout.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::print_to(&mut *$crate::io::stdout(), format_args!($($arg)*), "stdout")
    };
}

/// Write a formatted string to stdout and terminate with a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($args:tt)*) => ($crate::print!("{}\n", format_args!($($args)*)));
}
