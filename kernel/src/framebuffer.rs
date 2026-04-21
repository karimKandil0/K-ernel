use crate::font::draw_char;

pub fn draw_str(ptr: *mut u32, pitch: u64, bpp: u16, mut x: usize, y: usize, s: &str) -> () {
    for char in s.bytes() {
        draw_char(ptr, pitch, bpp, x, y, char);
        x += 8;
    }
}
