#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod arch;
mod drivers;
mod memory;
mod shell;
mod storage;
mod sync;
mod scheduler;

use drivers::framebuffer::Writer;
use limine::request::FramebufferRequest;

// Limine framebuffer request — filled before _start runs
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER: FramebufferRequest = FramebufferRequest::new();

// Global writer — None until framebuffer is initialized
pub static mut WRITER: Option<Writer> = None;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    print!("\nPANIC: {}\n", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // CPU infrastructure — must come first
    arch::x86_64::gdt::init();
    arch::x86_64::idt::init();
    arch::x86_64::pit::init();
    drivers::keyboard::init();
    x86_64::instructions::interrupts::enable();

    // Initialize framebuffer writer
    if let Some(response) = FRAMEBUFFER.response() {
        if let Some(fb) = response.framebuffers().first() {
            let ptr = fb.address() as *mut u32;
            let writer = Writer::new(ptr, fb.pitch, fb.bpp, 0, 0, fb.width as usize, fb.height as usize);
            unsafe { WRITER = Some(writer); }
            print!("K-ernel\n");
        }
    }

    // Memory subsystem — frame allocator + heap mapping
    memory::frame_allocator::init();
    memory::paging::init();

    // Storage — find AHCI controller, init first SATA port
    unsafe {
        if let Some(dev) = storage::pci::find_ahci() {
            print!("pci: AHCI found at bus={} device={}\n", dev.bus, dev.device);
            storage::ahci::find_and_init(&dev);
        } else {
            print!("pci: no AHCI controller found\n");
        }
    }

    shell::init();

    loop {}
}

// print! macro — writes formatted text through the global Writer
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        unsafe {
            if let Some(ref mut w) = crate::WRITER {
                use core::fmt::Write;
                let _ = w.write_fmt(format_args!($($arg)*));
            }
        }
    }
}
