static FONT: &[u8] = include_bytes!("font.psf");

pub fn draw_char(ptr: *mut u32, pitch: u64, bpp: u16, x: usize, y: usize, c: u8) -> () {
    let glyph_offset = 32 + (c as usize * 16);
    let glyph = &FONT[glyph_offset..];

    for row_idx in 0..16 {
        let row_byte = glyph[row_idx];
        for bit in 0..8 {
            if row_byte & (0x80 >> bit) != 0 {
                let px = x + bit;
                let py = y + row_idx;
                let offset = (py * pitch as usize + px * (bpp as usize / 8)) / 4;
                unsafe {
                    ptr.add(offset).write_volatile(0x00FF0000);
                }
           }
        }
    }
    
}
