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

const TEST_PREFIX: &'static str = "[integration_test_skeleton]";

#[test_case]
fn test_integration() {
    serial_print!("{} test_integration... ", TEST_PREFIX);
    // Do stuff here.
    serial_println!("[ok]");
}
