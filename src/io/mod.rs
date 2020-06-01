use spin::MutexGuard;

pub mod vga_text;

use vga_text::Writer;

pub fn stdout<'a>() -> MutexGuard<'a, Writer> {
    vga_text::WRITER.lock()
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => ({
        use core::fmt::Write;
        use $crate::io;

        write!(io::stdout(), "{}", format_args!($($args)*)).unwrap()
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($args:tt)*) => ($crate::print!("{}\n", format_args!($($args)*)));
}
