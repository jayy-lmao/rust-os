#![no_std]
#![no_main] // disable rust entry point
#![feature(exclusive_range_pattern)]


use core::panic::PanicInfo;
mod vga_buffer;


#[no_mangle] // dont mangle names
pub extern "C" fn _start() -> ! {
  println!("Hello + {:?}", "Woop");

  loop{}
}

// Panic func
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}