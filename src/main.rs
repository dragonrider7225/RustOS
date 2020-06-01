//! An executable wrapper around `lib.rs`

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rust_os::println;

use rust_os::io::{self, vga_text::{BackgroundColor, CharColor, TextColor}};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::panic(info)
}

/// The entry point for the binary.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let text = "Hello, World!";
    let mut counter = 0;
    io::stdout().set_color(CharColor::from((BackgroundColor::SOLID_BLACK, TextColor::LIGHT_CYAN)));
    println!("Solid black, light cyan");
    io::stdout().set_color(CharColor::from((BackgroundColor::BLINK_BLACK, TextColor::LIGHT_CYAN)));
    println!("Blink black, light cyan");
    while counter < 10 {
        io::stdout().set_color(CharColor(0x7b ^ ((counter & 0xF) << 4) as u8));
        println!("{}", text);
        counter += 1;
        for _ in 0..=1_300_000 {}
    }
    panic!()
}
