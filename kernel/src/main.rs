#![no_std]
#![no_main]

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buf: *mut u8 = 0xB8000 as *mut u8;

    unsafe {
        vga_buf.write_volatile(b'K');
        vga_buf.add(1).write_volatile(0x0F);
    }

    loop { }
}
