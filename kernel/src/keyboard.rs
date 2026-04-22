use pic8259::ChainedPics;
use core::ptr::addr_of_mut;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::instructions::port::PortReadOnly;
use crate::print;
use crate::WRITER;

// PIC vector offsets — remap IRQs away from CPU exception vectors (0-31)
pub const PIC_1_OFFSET: u8 = 32; // Master PIC: IRQ0-7 → vectors 32-39
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8; // Slave PIC: IRQ8-15 → vectors 40-47

pub static mut PICS: ChainedPics = unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) };

// PS/2 Set 1 scancode → ASCII lookup table.
// Index = scancode (make code only, < 128). Value = ASCII byte. 0 = non-printable.
const SCANCODE_MAP: [u8; 58] = [
    0,    0,    b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', 0,
    0,    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n',
    0,    b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`',
    0,    b'\\',b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/', 0,   0,   0,   b' ',
];

// IRQ1 handler — called by CPU on every keypress/release.
// Reads scancode from PS/2 data port, converts to ASCII, prints.
// Must send EOI (end of interrupt) or PIC will not fire further interrupts.
pub extern "x86-interrupt" fn keyboard_handler(_frame: InterruptStackFrame) {
    unsafe {
        let mut port: PortReadOnly<u8> = PortReadOnly::new(0x60);
        let scancode: u8 = port.read();

        // Only process make codes (key press). Break codes = make + 128.
        if scancode < 128 && (scancode as usize) < SCANCODE_MAP.len() {
            let ascii = SCANCODE_MAP[scancode as usize];
            if ascii != 0 {
                print!("{}", ascii as char);
            }
        }

        // Signal end of interrupt — required or PIC blocks further IRQs
        let pics = addr_of_mut!(PICS);
        (*pics).notify_end_of_interrupt(33);
    }
}

// Initialize and unmask PIC. Must be called before enabling CPU interrupts.
pub fn init() {
    unsafe {
        let pics = addr_of_mut!(PICS);
        (*pics).initialize(); // Remap IRQ vectors
        (*pics).write_masks(0b11111101, 0b11111111); // Enable IRQ1 (keyboard) only
    }
}
