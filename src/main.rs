#![no_std]
#![no_main] // disable rust entry point
#![feature(exclusive_range_pattern)]


use core::panic::PanicInfo;
mod vga_buffer;


static HELLO: &[u8] = b"Yetus my feetus!";
#[no_mangle] // dont mangle names
pub extern "C" fn _start() -> ! {
  vga_buffer::print_something();

  loop{}
}

// Panic func
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}