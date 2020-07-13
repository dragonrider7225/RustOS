use x86_64::{
    instructions::{segmentation, tables},
    structures::{gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector}, tss::TaskStateSegment},
    VirtAddr,
};

/// The index of the stack for handling [`DoubleFault`] in the Interrupt Stack Table.
///
/// [`DoubleFault`]: ../cpu_exception/enum.CpuException.html#variant.DoubleFault
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

/// The index of the stack for handling [`StackSegmentFault`] in the Interrupt Stack Table.
///
/// [`StackSegmentFault`]: ../cpu_exception/enum.CpuException.html#variant.PageFault
pub const STACK_SEGMENT_FAULT_IST_INDEX: u16 = 1;

/// The index of the stack for handling [`PageFault`] in the Interrupt Stack Table.
///
/// [`PageFault`]: ../cpu_exception/enum.CpuException.html#variant.PageFault
pub const PAGE_FAULT_IST_INDEX: u16 = 2;

macro_rules! make_stack {
    ($size:expr) => {{
        static mut STACK: [u8; $size] = [0; $size];

        let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
        let stack_end = stack_start + $size;
        stack_end
    }}
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        const STACK_SIZE: usize = 4096;

        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = make_stack!(STACK_SIZE);
        tss.interrupt_stack_table[STACK_SEGMENT_FAULT_IST_INDEX as usize] = make_stack!(STACK_SIZE);
        tss.interrupt_stack_table[PAGE_FAULT_IST_INDEX as usize] = make_stack!(STACK_SIZE);
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

/// Set up the Global Descriptor Table.
pub fn init() {
    GDT.0.load();
    unsafe {
        segmentation::set_cs(GDT.1.code_selector);
        tables::load_tss(GDT.1.tss_selector);
    }
}
