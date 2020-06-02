use core::fmt::{Arguments, Write};

use spin::MutexGuard;

pub mod serial;

pub mod vga_text;

#[cfg(not(test))]
mod _impl {
    use super::*;
    use vga_text::Writer;

    pub fn stdout<'a>() -> MutexGuard<'a, Writer> {
        vga_text::WRITER.lock()
    }

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

    pub fn stdout<'a>() -> MutexGuard<'a, SerialPort> {
        serial::SERIAL1.lock()
    }

    #[macro_export]
    macro_rules! set_stdout_color {
        ($color:expr) => {{
            let _ignore = $color;
        }};
    }
}

pub use _impl::stdout;

#[doc(hidden)]
pub fn print_to(out: &mut dyn Write, args: Arguments, name: &str) {
    out.write_fmt(args)
        .unwrap_or_else(|_| panic!("Failed to write to {}: {}", name, args));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::print_to(&mut *$crate::io::stdout(), format_args!($($arg)*), "stdout")
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($args:tt)*) => ($crate::print!("{}\n", format_args!($($args)*)));
}
