//! The actual implementation of the OS.

#![no_std]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(panic_info_message)]

use core::panic::PanicInfo;

pub mod io;

use io::vga_text::CharColor;

/// A function that can handle a panic. This is the simplest way to never return.
pub fn panic(info: &PanicInfo) -> ! {
    io::stdout().set_color(CharColor(0x0b));
    println!("{}", info);
    loop {}
}
