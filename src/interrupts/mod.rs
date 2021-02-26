mod idt;
use lazy_static::lazy_static;
use crate::println;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}


lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        idt.set_handler(0, divide_by_zero_wrapper);
        idt
    };
}


#[naked]
extern "C" fn divide_by_zero_wrapper() -> ! {
    unsafe {
        llvm_asm!("mov rdi, rsp; call $0"
            :: "i"(divide_by_zero_handler as extern "C" fn(_) -> !)
            : "rdi" : "intel");
        ::core::intrinsics::unreachable();
    }
}

extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) -> !
{
    println!("\nEXCEPTION: DIVIDE BY ZERO\n{:#?}", unsafe { &*stack_frame });
    loop {}
}

pub fn init(){
    println!("load idt");
    IDT.load();
}