use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = make_idt();
}

/// Set up the Interrupt Descriptor Table.
pub fn init_idt() {
    IDT.load();
}

fn make_idt() -> InterruptDescriptorTable {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    unsafe {
        idt.double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX);
    }
    unsafe {
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler)
            .set_stack_index(crate::gdt::STACK_SEGMENT_FAULT_IST_INDEX);
    }
    unsafe {
        idt.page_fault.set_handler_fn(page_fault_handler)
            .set_stack_index(crate::gdt::PAGE_FAULT_IST_INDEX);
    }
    idt
}

extern "x86-interrupt" fn breakpoint_handler(frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", frame)
}

extern "x86-interrupt" fn double_fault_handler(frame: &mut InterruptStackFrame, _: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", frame)
}

extern "x86-interrupt" fn stack_segment_fault_handler(frame: &mut InterruptStackFrame, _: u64) {
    panic!("EXCEPTION: STACK SEGMENT FAULT\n{:#?}", frame)
}

extern "x86-interrupt" fn page_fault_handler(
    frame: &mut InterruptStackFrame,
    _: PageFaultErrorCode,
) {
    panic!("EXCEPTION: PAGE FAULT\n{:#?}", frame)
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
