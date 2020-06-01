//! The actual implementation of the OS.

#![no_std]

use core::panic::PanicInfo;

/// A function that can handle a panic. This is the simplest way to never return.
pub fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[inline]
pub fn write_to_vga_buffer(s: &[u8]) {
    let mut vga_buffer = 0xb_8000 as *mut u8;
    for &byte in s.iter() {
        unsafe {
            *vga_buffer = byte;
            vga_buffer = vga_buffer.offset(1);
            *vga_buffer = 0xb;
            vga_buffer = vga_buffer.offset(1);
        }
    }
}
