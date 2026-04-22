use pic8259::ChainedPics;
use core::ptr::addr_of_mut;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::instructions::port::PortReadOnly;
use crate::WRITER;
use crate::print;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static mut PICS: ChainedPics = unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) };

const SCANCODE_MAP: [u8; 58] = [
      0, 0, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', 0,
      0, b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n',
      0, b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`',
      0, b'\\', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/', 0, 0, 0, b' ',
  ];

pub extern "x86-interrupt" fn keyboard_handler(_frame: InterruptStackFrame) {
    unsafe {
        let pics = addr_of_mut!(PICS);
        let mut port: PortReadOnly<u8> = PortReadOnly::new(0x60);
        let scancode: u8 = unsafe { port.read() };
        if scancode < 128 {
            let ascii = SCANCODE_MAP[scancode as usize];
            if ascii != 0 {
                print!("{}", ascii as char);
            }
        }
        (*pics).notify_end_of_interrupt(33);
    }
}

pub fn init() {
    unsafe {
        let pics = addr_of_mut!(PICS);
        (*pics).initialize();
        (*pics).write_masks(0b11111101, 0b11111111);
    }
}
