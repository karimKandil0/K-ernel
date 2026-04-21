#![no_std]
#![no_main]
mod font;
mod framebuffer;
use framebuffer::Writer;
use limine::request::FramebufferRequest;
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER: FramebufferRequest = FramebufferRequest::new();

static mut WRITER: Option<Writer> = None;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    if let Some(response) = FRAMEBUFFER.response() {

        if let Some(fb) = response.framebuffers().first() {

            let ptr = fb.address() as *mut u32;
            let writer: Writer = Writer::new(ptr, fb.pitch, fb.bpp, 0, 0, fb.width as usize, fb.height as usize);

            unsafe {
                WRITER = Some(writer);
            }
            
            print!("K-ernel says: fuck you");
        }
    }

    loop { }
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
