# Memory-Mapped I/O

## What is it?
Hardware registers exposed as memory addresses. Read/write to those addresses = talk to hardware.

## VGA case
VGA text buffer lives at physical address `0xB8000`. Writing bytes there makes text appear on screen.

## Rust implications
- Must use raw pointers (`*mut u8`)
- Must use `unsafe` — compiler can't verify hardware memory is valid
- Must use `volatile` writes — compiler must not optimize them away (no "dead store elimination")

## Key concept
`volatile` = "this write has side effects the compiler can't see, don't optimize it"

## Related
- [[02-VGA/VGA-Buffer]]
- [[Concepts/Unsafe-Rust]]

## Status
> Not started
