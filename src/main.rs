#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello world!");

    os::init();
    x86_64::instructions::interrupts::int3();

    println!("didn't crush! dependence");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
