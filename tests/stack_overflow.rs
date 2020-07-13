//! A test that the GDT enables an exception handler to be called on stack overflow.

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rust_os;
use rust_os::qemu::{self, QemuExitCode};

use volatile::Volatile;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(rust_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

#[no_mangle]
extern "C" fn _start() -> ! {
    serial_print!("[stack_overflow]... ");
    rust_os::gdt::init();
    init_test_idt();
    stack_overflow();
    panic!("Execution continued after stack overflow");
}

extern "x86-interrupt" fn double_fault_handler(_: &mut InterruptStackFrame, _: u64) -> ! {
    serial_println!("[ok]");
    qemu::exit_qemu(QemuExitCode::Success)
}

fn init_test_idt() {
    TEST_IDT.load();
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    // Add a frame to the stack.
    stack_overflow();
    // Add an operation that the compiler can't optimize out to prevent it from turning the
    // infinite recursion into `loop {}`.
    Volatile::new(0).read();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic(info)
}
