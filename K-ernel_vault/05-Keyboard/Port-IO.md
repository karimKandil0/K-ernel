# Port I/O

## What is it?
x86 has a separate I/O address space (not memory-mapped). Access via `in`/`out` instructions.

## Rust access
No safe way — requires `unsafe` + inline assembly or `x86_64` crate's `Port` type.

```rust
use x86_64::instructions::port::Port;
let mut port: Port<u8> = Port::new(0x60);
let scancode = unsafe { port.read() };
```

## Key ports
- `0x60` — PS/2 data port (keyboard scancode)
- `0x64` — PS/2 status/command port
- `0x20`, `0xA0` — PIC (interrupt controller) command ports
- `0x21`, `0xA1` — PIC mask registers

## PIC (Programmable Interrupt Controller)
Must remap PIC IRQs from vectors 0–15 to 32–47 (avoid collision with CPU exceptions).

## Related
- [[05-Keyboard/PS2-Protocol]]
- [[03-Interrupts/IDT]]

## Status
> Not started
