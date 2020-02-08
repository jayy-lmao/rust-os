#![no_std]
#![no_main] // disable rust entry point

use core::panic::PanicInfo;

#[no_mangle] // dont mangle names
pub extern "C" fn _start() -> ! {
  loop {}
}


// Panic func
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}