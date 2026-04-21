# VGA Buffer

## Layout
80 columns x 25 rows. Each cell = 2 bytes:
- Byte 0: ASCII character
- Byte 1: color attribute (foreground + background, 4 bits each)

Total: 80 * 25 * 2 = 4000 bytes at `0xB8000`

## Color byte
```
Bits 7-4: background color
Bits 3-0: foreground color
```
Colors: Black=0, Blue=1, Green=2, Cyan=3, Red=4, Magenta=5, Brown=6, LightGray=7...

## What we'll build
- `VgaBuffer` struct wrapping the raw memory
- `Writer` struct with cursor tracking + color
- `write_byte`, `write_string`, newline handling
- Eventually: `print!` macro using `core::fmt::Write`

## Related
- [[02-VGA/Memory-Mapped-IO]]

## Status
> Not started
