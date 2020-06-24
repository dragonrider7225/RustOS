use core::convert::TryFrom;

/// Tools related to handling interrupts.
pub mod interrupts;

/// An exception thrown by the CPU.
/// The documentation on the various exceptions is based on the page at
/// [the OSDev Wiki](https://wiki.osdev.org/Exceptions).
///
/// # Error Codes
/// When an exception includes an error code which represents a segment selector, the bits
/// represent the following:
/// * Bit 0 is set if and only if the exception originated externally to the processor.
/// * Bit 1 is set if and only if the exception is related to a descriptor in the Interrupt
/// Descriptor Table.
/// * Bit 2 is irrelevant when bit 1 is set, but if bit 1 is cleared, bit 2 is set when the
/// exception is related to a descriptor in the Local Descriptor Table and cleared when the
/// exception is related to a descriptor in the Global Descriptor Table.
/// * Bits 15-3 are the index into the appropriate Descriptor Table as determined by bits 2-1.
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum CpuException {
    /// The result of executing `DIV` or `IDIV` with 0 as the denominator. Sometimes also thrown
    /// when the result of the instruction is too large to fit in the destination.
    /// # Saved Instruction
    /// The saved instruction is the instruction which caused the exception.
    DivideByZero = 0x00,
    /// The result of
    /// * an instruction fetch breakpoint, or
    /// * a general detect condition, or
    /// * a data read or write breakpoint, or
    /// * an I/O read or write breakpoint, or
    /// * a single-step, or
    /// * a task-switch.
    /// In the first two cases, the exception is a Fault. In all other cases, the exception is a
    /// Trap.
    /// # Saved Instruction
    /// When the exception is a fault, the saved instruction is the instruction which caused the
    /// exception.
    ///
    /// When the exception is a trap, the saved instruction is the instruction after the
    /// instruction which caused the exception.
    /// # Error Code
    /// Although the exception does not push an error code, the debug registers contain information
    /// about the exception.
    Debug = 0x01,
    /// TODO: Add documentation.
    NonMaskableInterrupt = 0x02,
    /// The result of the `INT3` instruction. Some debug software implements the insertion of
    /// breakpoints by replacing the targeted instruction with `INT3` until the instruction is
    /// reached, at which point it replaces the interrupt with the original instruction and
    /// decrements the instruction pointer.
    /// # Saved Instruction
    /// The saved instruction is the instruction after the interrupt.
    Breakpoint = 0x03,
    /// The result of
    /// * executing the `INTO` instruction while the Overflow bit in `RFLAGS` is set, or
    /// * executing an integer division instruction whose result is too large to fit in the target
    /// (int::MIN / -1).
    /// # Saved Instruction
    /// The saved instruction is the instruction after the `INTO` instruction or the integer
    /// division instruction that overflowed.
    Overflow = 0x04,
    /// The result of executing `BOUND` with an index outside of the array.
    /// # Saved Instruction
    /// The saved instruction is the instruction which caused the exception.
    BoundRangeExceeded = 0x05,
    /// The result of a failure to parse an instruction.
    /// # Saved Instruction
    /// The saved instruction is the invalid instruction.
    InvalidOpcode = 0x06,
    /// The result of executing an FPU instruction when there is no FPU or the FPU has been
    /// disabled by a flag in `CR0`.
    /// # Saved Instruction
    /// The saved instruction is the instruction which caused the exception.
    DeviceNotAvailable = 0x07,
    /// The result of any exception for which no handler could be called.
    /// # Saved Instruction
    /// The saved instruction is undefined. A process which double faults *cannot* be recovered and
    /// must be terminated.
    /// # Error Code
    /// The error code is always 0.
    DoubleFault = 0x08,
    /// A general protection fault from an external FPU.
    #[deprecated(note = "Not applicable with integrated FPU.")]
    CoprocessorSegmentOverrun = 0x09,
    /// The result of an attempt to reference an invalid stack-segment selector.
    /// # Saved Instruction
    /// Usually, the saved instruction is the first instruction of the new task. However, if the
    /// exception occurred before loading the segment selectors from the TSS, the saved instruction
    /// is instead the instruction which caused the exception.
    /// # Error Code
    /// The error code is a selector index.
    InvalidTss = 0x0A,
    /// The result of an attempt to load a segment or gate which is not present unless the load is
    /// the result of resolving a stack-segment reference, in which case `StackSegmentFault` is
    /// raised instead.
    /// # Saved Instruction
    /// The saved instruction is the instruction which caused the exception.
    /// # Error Code
    /// The error code is the segment selector index of the segment descriptor which caused the
    /// exception.
    SegmentNotPresent = 0x0B,
    /// The result of
    /// * loading a stack-segment referencing a segment descriptor which is not present, or
    /// * the execution of any `PUSH` or `POP` instruction while the stack address is not in
    /// canonical form, or
    /// * the execution of any instruction using `ESP` or `EBP` as a base register while the stack
    /// address is not in canonical form, or
    /// * failure of the stack-limit check.
    /// `StackSegmentFault` is separate from `SegmentNotPresent` because the latter pushes `EIP`,
    /// `CS`, `EFLAGS`, `ESP`, and `SS` to the stack, which can't be done under any condition which
    /// would raise `StackSegmentFault`.
    /// # Saved Instruction
    /// The saved instruction is the instruction which caused the exception.
    /// # Error Code
    /// The error code is the stack-segment selector index in the first case.
    /// The error code is 0 in all other cases.
    StackSegmentFault = 0x0C,
    /// A `GeneralProtectionFault` is the result of taking an action without having the associated
    /// privilege.
    /// For example:
    /// * segment error, or
    /// * executing a privileged instruction with non-zero `CPL`, or
    /// * writing a 1 in a reserved register field, or
    /// * referencing or accessing a null-descriptor, or
    /// * trying to access an unimplemented register, e.g., `CR7`.
    /// # Saved Instruction
    /// The saved instruction is the instruction which caused the exception.
    /// # Error Code
    /// The error code is the segment selector index when applicable.
    /// The error code is 0 in all other cases.
    GeneralProtectionFault = 0x0D,
    /// The result of
    /// * an attempt to access a page directory entry or page table entry that is not present in
    /// physical memory, or
    /// * an attempt to load the instruction TLB with a translation for a non-executable page, or
    /// * a failed protection check, or
    /// * an invalid attempt to access a page directory entry or page table entry with its reserved
    /// bit set to 1.
    /// # Saved Instruction
    /// The saved instruction is the instruction which caused the exception.
    /// # Error Code
    /// The five least-significant bits are set or cleared as follows:
    /// * Bit 0 is the Present bit. When set, the page fault was caused by a page-protection
    /// violation. When cleared, the page fault was caused by a non-present page.
    /// * Bit 1 is the Write bit. When set, the page fault was caused by a write access. When
    /// cleared, the page fault was caused by a read access.
    /// * Bit 2 is the User bit. When set, the page fault was caused while `CPL` was 3. Note that
    /// this does not necessarily mean that no page fault would have occurred if `CPL` were 0.
    /// * Bit 3 is the Reserved Write bit. When set, one or more relevant page directory entries
    /// have their reserved bits set. This only applies when the `PSE` or `PAE` flags in `CR4` are
    /// set.
    /// * Bit 4 is the Instruction Fetch bit. When set, the page fault was caused by an instruction
    /// fetch. This only applies when the No-Execute bit is supported and enabled.
    /// In addition to this error code, the processor will also set `CR2` to contain the virtual
    /// address which caused the page fault.
    PageFault = 0x0E,
    // 0x0F reserved for future use.
    /// The result of the execution of `FWAIT`, `WAIT`, or any waiting floating-point instruction
    /// while the Numeric Error bit of `CR0` is set and the exception bit in the x87 floating point
    /// status-word register is set.
    /// # Saved Instruction
    /// The saved instruction is the instruction which was about to be executed when the exception
    /// occurred. The x87 instruction pointer register contains the address of the last instruction
    /// which caused the exception.
    /// # Error Code
    /// Although the exception does not push an error code, the x87 status-word register contains
    /// information about the exception.
    X87FloatingPointException = 0x10,
    /// The result of an unaligned memory data reference while alignment checking is enabled and
    /// `CPL` is 3. To enable alignment checking, set the Alignment Mask bit in `CR0` and the
    /// Alignment Check bit in `RFLAGS`.
    /// # Saved Instruction
    /// The saved instruction is the instruction which caused the exception.
    AlignmentCheck = 0x11,
    /// The result of the processor detecting internal errors while the Machine Check Exception bit
    /// of `CR4` is set. The exact implementation of the control flow that throws this exception is
    /// model specific and is not required to exist at all.
    /// # Saved Instruction
    /// The saved instruction is implementation-dependent.
    /// # Error Code
    /// The location of additional information about the error is implementation-dependent.
    MachineCheck = 0x12,
    /// The result of an unmasked 128-bit media floating-point exception while the OSXMMEXCPT bit
    /// in `CR4` is set. If OSXMMEXCPT is not set, then this exception is replaced by Undefined
    /// Opcode.
    /// # Saved Instruction
    /// The saved instruction is the instruction which caused the exception.
    /// # Error Code
    /// Although the exception does not push an error code, `MXCSR` contains information about the
    /// exception.
    SimdFloatingPointException = 0x13,
    /// TODO: Add documentation.
    VirtualizationException = 0x14,
    // 0x15..=0x1D reserved for future use.
    /// TODO: Add documentation.
    SecurityException = 0x1E,
    // 0x1F reserved for future use.
}

impl CpuException {
    /// An exception is an *abort* if the process that threw it cannot be meaningfully recovered
    /// under any circumstances. The only exceptions that are abort are `DoubleFault` and
    /// `MachineCheck` (and `TripleFault`, but that is only ever handled by resetting the processor
    /// and thus cannot be thrown and does not require a representation).
    pub fn is_abort(&self) -> bool {
        match self {
            Self::DivideByZero => false,
            Self::Debug => false,
            Self::NonMaskableInterrupt => false,
            Self::Breakpoint => false,
            Self::Overflow => false,
            Self::BoundRangeExceeded => false,
            Self::InvalidOpcode => false,
            Self::DeviceNotAvailable => false,
            Self::DoubleFault => true,
            #[allow(deprecated)]
            Self::CoprocessorSegmentOverrun => false,
            Self::InvalidTss => false,
            Self::SegmentNotPresent => false,
            Self::StackSegmentFault => false,
            Self::GeneralProtectionFault => false,
            Self::PageFault => false,
            Self::X87FloatingPointException => false,
            Self::AlignmentCheck => false,
            Self::MachineCheck => true,
            Self::SimdFloatingPointException => false,
            Self::VirtualizationException => false,
            Self::SecurityException => false,
        }
    }

    /// An exception is a *fault* if the problem that caused it can be corrected and the process
    /// can continue as if the exception had never been thrown. The faults are `DivideByZero`,
    /// `Debug` (sometimes), `BoundRangeExceeded`, `InvalidOpcode`, `DeviceNotAvailable`,
    /// `CoprocessorSegmentOverrun`, `InvalidTss`, `SegmentNotPresent`, `StackSegmentFault`,
    /// `GeneralProtectionFault`, `PageFault`, `X87FloatingPointException`, `AlignmentCheck`,
    /// `SimdFloatingPointException`, and `VirtualizationException`.
    pub fn is_fault(&self) -> bool {
        match self {
            Self::DivideByZero => true,
            Self::Debug => true,
            Self::NonMaskableInterrupt => false,
            Self::Breakpoint => false,
            Self::Overflow => false,
            Self::BoundRangeExceeded => true,
            Self::InvalidOpcode => true,
            Self::DeviceNotAvailable => true,
            Self::DoubleFault => false,
            #[allow(deprecated)]
            Self::CoprocessorSegmentOverrun => true,
            Self::InvalidTss => true,
            Self::SegmentNotPresent => true,
            Self::StackSegmentFault => true,
            Self::GeneralProtectionFault => true,
            Self::PageFault => true,
            Self::X87FloatingPointException => true,
            Self::AlignmentCheck => true,
            Self::MachineCheck => false,
            Self::SimdFloatingPointException => true,
            Self::VirtualizationException => true,
            Self::SecurityException => false,
        }
    }

    /// The exception is caused by a hardware interrupt instead of something the program did. The
    /// only such exception is `NonMaskableInterrupt`
    pub fn is_interrupt(&self) -> bool {
        match self {
            Self::DivideByZero => false,
            Self::Debug => false,
            Self::NonMaskableInterrupt => true,
            Self::Breakpoint => false,
            Self::Overflow => false,
            Self::BoundRangeExceeded => false,
            Self::InvalidOpcode => false,
            Self::DeviceNotAvailable => false,
            Self::DoubleFault => false,
            #[allow(deprecated)]
            Self::CoprocessorSegmentOverrun => false,
            Self::InvalidTss => false,
            Self::SegmentNotPresent => false,
            Self::StackSegmentFault => false,
            Self::GeneralProtectionFault => false,
            Self::PageFault => false,
            Self::X87FloatingPointException => false,
            Self::AlignmentCheck => false,
            Self::MachineCheck => false,
            Self::SimdFloatingPointException => false,
            Self::VirtualizationException => false,
            Self::SecurityException => false,
        }
    }

    /// An exception is a *trap* if the instruction that caused it is allowed to complete normally
    /// *before* the exception is thrown. The traps are `Debug` (sometimes), `Breakpoint`, and
    /// `Overflow`.
    pub fn is_trap(&self) -> bool {
        match self {
            Self::DivideByZero => false,
            Self::Debug => true,
            Self::NonMaskableInterrupt => false,
            Self::Breakpoint => true,
            Self::Overflow => true,
            Self::BoundRangeExceeded => false,
            Self::InvalidOpcode => false,
            Self::DeviceNotAvailable => false,
            Self::DoubleFault => false,
            #[allow(deprecated)]
            Self::CoprocessorSegmentOverrun => false,
            Self::InvalidTss => false,
            Self::SegmentNotPresent => false,
            Self::StackSegmentFault => false,
            Self::GeneralProtectionFault => false,
            Self::PageFault => false,
            Self::X87FloatingPointException => false,
            Self::AlignmentCheck => false,
            Self::MachineCheck => false,
            Self::SimdFloatingPointException => false,
            Self::VirtualizationException => false,
            Self::SecurityException => false,
        }
    }
}

impl TryFrom<u8> for CpuException {
    type Error = &'static str;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        match code {
            0x00 => Ok(Self::DivideByZero),
            0x01 => Ok(Self::Debug),
            0x02 => Ok(Self::NonMaskableInterrupt),
            0x03 => Ok(Self::Breakpoint),
            0x04 => Ok(Self::Overflow),
            0x05 => Ok(Self::BoundRangeExceeded),
            0x06 => Ok(Self::InvalidOpcode),
            0x07 => Ok(Self::DeviceNotAvailable),
            0x08 => Ok(Self::DoubleFault),
            0x09 => {
                #[allow(deprecated)]
                let ret = Ok(Self::CoprocessorSegmentOverrun);
                ret
            }
            0x0A => Ok(Self::InvalidTss),
            0x0B => Ok(Self::SegmentNotPresent),
            0x0C => Ok(Self::StackSegmentFault),
            0x0D => Ok(Self::GeneralProtectionFault),
            0x0E => Ok(Self::PageFault),
            0x0F => Err("0x0F is reserved"),
            0x10 => Ok(Self::X87FloatingPointException),
            0x11 => Ok(Self::AlignmentCheck),
            0x12 => Ok(Self::MachineCheck),
            0x13 => Ok(Self::SimdFloatingPointException),
            0x14 => Ok(Self::VirtualizationException),
            0x15 => Err("0x15 is reserved"),
            0x16 => Err("0x16 is reserved"),
            0x17 => Err("0x17 is reserved"),
            0x18 => Err("0x18 is reserved"),
            0x19 => Err("0x19 is reserved"),
            0x1A => Err("0x1A is reserved"),
            0x1B => Err("0x1B is reserved"),
            0x1C => Err("0x1C is reserved"),
            0x1D => Err("0x1D is reserved"),
            0x1E => Ok(Self::SecurityException),
            0x1F => Err("0x1F is reserved"),
            _ => Err("Invalid exception number"),
        }
    }
}
