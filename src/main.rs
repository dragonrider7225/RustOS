//! An executable wrapper around `lib.rs`

#![no_std]
#![no_main]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(panic_info_message)]
// Required for `cargo xtest`.
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[macro_use]
extern crate rust_os;

use rust_os::qemu::{self, QemuExitCode};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if cfg!(test) {
        rust_os::test_panic(info)
    } else {
        rust_os::no_test_panic(info)
    }
}

/// The entry point for the binary.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    rust_os::init();
    rust_os::draw_vga_test();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    // TODO: event loop

    qemu::exit_qemu(QemuExitCode::Success)
}
