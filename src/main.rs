#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(asm)]
#![feature(llvm_asm)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]

mod vga_buffer;
use core::panic::PanicInfo;
mod interrupts;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    //println!("Hello world!");

    interrupts::init();
    raise_div_int();
    println!("didn't crush! dependence");
    loop {}
}

fn raise_div_int() {
    unsafe { asm!("mov dx, 0", "div dx") };
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
