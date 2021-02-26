mod idt;
use crate::println;
use lazy_static::lazy_static;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!("mov rdi, rsp",
                "sub rsp, 8",
                "call {}",sym $name);
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        idt.set_handler(0, handler!(divide_by_zero_handler));
        idt.set_handler(6, handler!(invalid_opcode_handler));
        idt
    };
}

extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) -> ! {
    let stack_frame = unsafe { &*stack_frame };
    println!(
        "\nException: divide by zero at {:#x}\n{:#?}",
        stack_frame.instruction_pointer, stack_frame
    );
    loop {}
}

extern "C" fn invalid_opcode_handler(stack_frame: &ExceptionStackFrame) -> ! {
    let stack_frame = unsafe { &*stack_frame };
    println!(
        "\nException: invalid opcode at {:#x}\n{:#?}",
            stack_frame.instruction_pointer, stack_frame
    );

    loop {}
}

pub fn init() {
    println!("load IDT");
    IDT.load();
}
