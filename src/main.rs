//! An executable wrapper around `lib.rs`

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::panic(info)
}

/// The entry point for the binary.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    rust_os::write_to_vga_buffer(b"Hello, World!");
    loop {}
}
