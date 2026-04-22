#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod allocator;
mod font;
mod framebuffer;
mod gdt;
mod idt;
mod memory;
mod paging;
mod keyboard;

use framebuffer::Writer;
use limine::request::FramebufferRequest;
use alloc::vec::Vec;

#[unsafe(link_section = ".requests")]
static FRAMEBUFFER: FramebufferRequest = FramebufferRequest::new();

pub static mut WRITER: Option<Writer> = None;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    gdt::init();
    idt::init();
    keyboard::init();
    x86_64::instructions::interrupts::enable();

    if let Some(response) = FRAMEBUFFER.response() {
        if let Some(fb) = response.framebuffers().first() {
            let ptr = fb.address() as *mut u32;
            let writer = Writer::new(ptr, fb.pitch, fb.bpp, 0, 0, fb.width as usize, fb.height as usize);
            unsafe { WRITER = Some(writer); }
            print!("K-ernel.\n");
            print!("h={}", fb.height);
        }
    }


    memory::init();
    paging::init();

    let mut v: Vec<u32> = Vec::new();
    v.push(1);
    v.push(2);
    print!("heap works: {}\n", v[0]);

    loop {}
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        unsafe {
            if let Some(ref mut w) = WRITER {
                use core::fmt::Write;
                let _ = w.write_fmt(format_args!($($arg)*));
            }
        }
    }
}
