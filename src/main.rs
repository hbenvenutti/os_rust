#![no_std]

use core::panic::PanicInfo;
// disables the standard library. A OS cannot use a os abstraction library.

/// This function is called on panic.
// #[panic_handler]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
fn main() {}
