#![no_std]
#![no_main]
mod font;
mod framebuffer;
use framebuffer::draw_str;
use limine::request::FramebufferRequest;
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER: FramebufferRequest = FramebufferRequest::new();

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    if let Some(response) = FRAMEBUFFER.response() {

        if let Some(fb) = response.framebuffers().first() {

            let ptr = fb.address() as *mut u32;
            
            draw_str(ptr, fb.pitch, fb.bpp, 10, 10 , "K-ernel");
        }
    }

    loop { }
}


