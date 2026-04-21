# PS/2 Protocol

## What is it?
Serial protocol for keyboards (and mice). Legacy but universally emulated by QEMU.

## How it works
Keyboard sends "scancodes" — one byte per key event (press or release).
- Press: scancode byte (e.g. `0x1E` = 'A' pressed)
- Release: `0xF0` prefix + scancode (Set 2) or `0x80+scancode` (Set 1)

## Scancode sets
- Set 1: original XT scancodes (most common in emulators)
- Set 2: AT scancodes (default on real hardware)

QEMU defaults to Set 1.

## How we receive them
PS/2 controller on port `0x60`. When key pressed → IRQ1 fires → read scancode from port `0x60`.

## Related
- [[05-Keyboard/Port-IO]]
- [[03-Interrupts/IDT]]

## Status
> Not started
