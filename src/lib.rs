//! The actual implementation of the OS.

#![no_std]

use core::panic::PanicInfo;

/// A function that can handle a panic. This is the simplest way to never return.
pub fn panic(_: &PanicInfo) -> ! {
    loop {}
}
