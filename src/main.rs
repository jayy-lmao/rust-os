#![no_std]
#![no_main] // disable rust entry point
#![feature(exclusive_range_pattern)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
  println!("Running {} tests", tests.len());
  for test in tests {
    test();
  }
}

use core::panic::PanicInfo;
mod vga_buffer;

#[test_case]
fn trivial_assertion() {
  print!("trivial assertion... ");
  assert_eq!(1, 1);
  println!("[ok]");
}

#[no_mangle] // dont mangle names
pub extern "C" fn _start() -> ! {
  println!("Hello + {:?}", "Woop");

  #[cfg(test)]
  test_main();

  loop {}
}

// Panic func
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  println!("{}", _info);

  loop {}
}
