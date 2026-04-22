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

    fn scroll(&mut self) {
        let row_size = (self.pitch / 4) as usize;
        unsafe {
            for row in 16..self.height {
                for col in 0..self.width {
                    let src = (row * row_size + col) as isize;
                    let dst = ((row - 16) * row_size + col) as isize;
                    let pixel = self.ptr.offset(src).read_volatile();
                    self.ptr.offset(dst).write_volatile(pixel);
                }
            }
            
            for row in (self.height - 16)..self.height {
                for col in 0..self.width {
                    let dst = (row * row_size + col) as isize;
                    self.ptr.offset(dst).write_volatile(0x00000000);
                }
            }
        }
    }

    pub fn draw_cursor(&mut self) {
        let row_size = (self.pitch / 4) as usize;
        unsafe {
            for row in 0..16 {
                let dst = ((self.y + row) * row_size + self.x) as isize;
                self.ptr.offset(dst).write_volatile(0x00FFFFFF);
            }
        }
    }

    pub fn clear_cursor(&mut self) {
        let row_size = (self.pitch / 4) as usize;
        unsafe {
            for row in 0..16 {
                let dst = ((self.y + row) * row_size + self.x) as isize;
                self.ptr.offset(dst).write_volatile(0x00000000);
            }
        }
    }

    pub fn print(&mut self, s: &str) {
        self.clear_cursor();
        for char in s.bytes() {
            if char == b'\n' {
                self.y += 16;
                if self.y + 16 >= self.height {
                    self.scroll();
                    self.y -= 16;
                }
                self.x = 0;
                continue
            } else {
                draw_char(self.ptr, self.pitch, self.bpp, self.x, self.y, char);
            }
            self.x += 8;
            if self.x + 8 >= self.width {
                self.y += 16;
                if self.y + 16 >= self.height {
                    self.scroll();
                    self.y -= 16;
                }
                self.x = 0;
                continue
            }
        }
        self.draw_cursor();
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}
