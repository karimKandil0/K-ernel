use crate::font::draw_char;

pub struct Writer {
    ptr: *mut u32,
    pitch: u64,
    bpp: u16,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Writer {
    pub fn new(ptr: *mut u32, pitch: u64, bpp: u16, x: usize, y: usize, width: usize, height: usize) -> Writer {
        Writer { ptr, pitch, bpp, x, y, width, height }
    }

    pub fn print(&mut self, s: &str) {
        for char in s.bytes() {
            draw_char(self.ptr, self.pitch, self.bpp, self.x, self.y, char);
            self.x += 8;
        }
    }
}
