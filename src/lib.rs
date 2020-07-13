//! A library which can be used to create an operating system.

#![no_std]
#![cfg_attr(test, no_main)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]

use core::panic::PanicInfo;

#[macro_use]
extern crate lazy_static;

/// Tools for input and output of bytes.
#[macro_use]
pub mod io;
use io::vga_text::{BackgroundColor, TextColor, Writer};

/// Tools for handling CPU exceptions.
pub mod cpu_exception;
use cpu_exception::interrupts;

/// Tools for handling the Global Descriptor Table.
pub mod gdt;

/// QEMU-specific functionality.
pub mod qemu;
use qemu::QemuExitCode;

/// Draws the available pairs of background and text colors.
pub fn draw_vga_test() {
    let old_color = io::vga_text::WRITER.lock().color();
    for bg_color in BackgroundColor::colors() {
        for text_color in TextColor::colors() {
            set_vga_color!((bg_color, text_color));
            vga_print!("X");
        }
        vga_println!();
    }
    set_vga_color!(old_color);
}

/// Initialize various parts of the OS.
pub fn init() {
    gdt::init();
    interrupts::init_idt();
}

/// The function to run the tests.
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    tests.iter().for_each(|test| test());
    serial_println!("All tests succeeded");

    qemu::exit_qemu(QemuExitCode::Success);
}

/// The panic implementation for the test framework.
pub fn test_panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);

    qemu::exit_qemu(QemuExitCode::Failure)
}

/// The panic implementation for when the panic message can be printed to stdout.
pub fn no_test_panic(info: &PanicInfo) -> ! {
    set_stdout_color!(Writer::DEFAULT_COLOR_PAIR);
    println!("{}\n", info);

    qemu::exit_qemu(QemuExitCode::Failure)
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic(info)
}

/// Entry point for `cargo xtest` for the library.
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}
