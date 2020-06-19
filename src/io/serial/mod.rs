use core::fmt::Arguments;

use lazy_static::lazy_static;

use spin::Mutex;

use uart_16550::SerialPort;

lazy_static! {
    /// A reference to the serial port at address `0x03F8`.
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    super::print_to(&mut *SERIAL1.lock(), args, "SERIAL1");
}

/// Write a formatted string to the first serial port.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::io::serial::_print(format_args!($($arg)*)));
}

/// Write a formatted string to the first serial port. Terminate with a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}
