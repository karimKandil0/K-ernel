use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::WRITER;
use crate::print;

static mut IDT: Option <InterruptDescriptorTable> = None;

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    print!("EXCEPTION: breakpoint\n");
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    print!("EXCEPTION: double fault\n");
    loop { }
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: x86_64::structures::idt::PageFaultErrorCode) {
    print!("EXCEPTION: page fault\n");
    loop { }
}

extern "x86-interrupt" fn general_protection_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    print!("EXCEPTION: general protection fault\n");
    loop { }
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    print!("EXCEPTION: invalid opcode\n");
    loop { }
}

pub extern "x86-interrupt" fn timer_handler(_frame: InterruptStackFrame) {
    unsafe {
        let pics = core::ptr::addr_of_mut!(crate::drivers::keyboard::PICS);
        (*pics).notify_end_of_interrupt(32);
    }
}

pub fn init() {
    unsafe {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt[33].set_handler_fn(crate::drivers::keyboard::keyboard_handler);
        idt[32].set_handler_fn(timer_handler);
        IDT = Some(idt);
        if let Some(ref i) = IDT {
            i.load();
        }
    }
}
