
/// An exit code for exiting QEMU.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    /// The exit code for when the kernel exits normally. Note that the success exit code is
    /// non-zero. This is because QEMU's `isa-debug-exit` device doubles whatever exit code it
    /// receives then adds 1 to it before making that code available to the caller. As a result, if
    /// `Success` were 0, the caller would be unable to distinguish between QEMU running the kernel
    /// and the kernel exiting normally with an actual exit code of `(0 << 1) | 1` and QEMU failing
    /// to run, exiting with an actual exit code of 1.
    Success = 0b10,
    /// The exit code for when the kernel exits in some way abnormally.
    Failure = 0b11,
}

/// Exit QEMU with the specified exit code.
pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;

    unsafe {
        Port::new(0xF4).write(exit_code as u32);
    }

    loop {}
}
