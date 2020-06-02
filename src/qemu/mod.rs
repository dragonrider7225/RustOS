#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0b10,
    Failure = 0b11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;

    unsafe {
        Port::new(0xF4).write(exit_code as u32);
    }

    loop {}
}
