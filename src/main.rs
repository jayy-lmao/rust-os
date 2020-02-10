#![no_std]
#![no_main] // disable rust entry point
use core::panic::PanicInfo;


static HELLO: &[u8] = b"Yetus my feetus!";
#[no_mangle] // dont mangle names
pub extern "C" fn _start() -> ! {
  let vga_buffer = 0xb8000 as *mut u8;

  for (i, &byte) in HELLO.iter().enumerate() {
    unsafe {
      *vga_buffer.offset(i as isize * 2) = byte;
      *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    }
  }

  loop{}
}

// Panic func
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}