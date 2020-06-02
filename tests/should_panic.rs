#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[macro_use]
extern crate rust_os;

use rust_os::qemu::{self, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    qemu::exit_qemu(QemuExitCode::Failure)
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        qemu::exit_qemu(QemuExitCode::Failure);
    }
    qemu::exit_qemu(QemuExitCode::Success);
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    serial_println!("[ok]");
    qemu::exit_qemu(QemuExitCode::Success)
}

const TEST_PREFIX: &'static str = "[should_panic]";

#[test_case]
fn should_panic() {
    serial_print!("{} should_panic... ", TEST_PREFIX);
    assert_eq!(0, 1);
}
