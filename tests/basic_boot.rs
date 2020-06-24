#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[macro_use]
extern crate rust_os;

use rust_os::qemu::{self, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    rust_os::init();

    test_main();

    qemu::exit_qemu(QemuExitCode::Success)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic(info)
}

const TEST_PREFIX: &'static str = "[basic_boot]";

/// Test that the VGA text buffer doesn't panic on write when the system hasn't been set up.
#[test_case]
fn test_vga_println_first() {
    serial_print!("{} test_vga_println_first... ", TEST_PREFIX);
    vga_println!("test_vga_println_first output");
    serial_println!("[ok]");
}
