#![no_std]
#![no_main]
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
            unsafe {
                for y in 0..100 {
                    for x in 0..100 {
                        let offset = (y * fb.pitch as usize + x * (fb.bpp as usize / 8)) / 4;
                        unsafe {
                            ptr.add(offset).write_volatile(0x0000FF00)
                        }
                    }
                }
            }
        }
    }

    loop { }
}
