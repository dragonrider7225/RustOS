use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

/// Set up the Interrupt Descriptor Table.
pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", frame)
}

#[cfg(test)]
mod test {
    const TEST_PREFIX: &'static str = "[rust_os::cpu_exception::interrupts]";

    #[test_case]
    fn test_breakpoint_exception() {
        serial_print!("{} test_breakpoint_exception... ", TEST_PREFIX);
        x86_64::instructions::interrupts::int3();
        serial_println!("[ok]");
    }
}
