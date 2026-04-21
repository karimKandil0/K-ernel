# Interrupt Descriptor Table (IDT)

## What is it?
A table of 256 entries. Each entry = handler for one interrupt/exception vector.

## How it works
CPU exception or IRQ fires → CPU looks up vector number in IDT → jumps to handler → handler runs → `iretq` returns

## Entry types
- Exception handlers (vectors 0–31) — CPU-generated (divide by zero, page fault, etc.)
- IRQ handlers (vectors 32+) — hardware interrupts (keyboard, timer)
- Software interrupts (`int` instruction)

## Key exceptions we'll handle
- `#DE` (0) — divide by zero
- `#PF` (14) — page fault
- `#GP` (13) — general protection fault
- Double fault (18) — fault while handling a fault

## Double fault
Special case — needs its own stack (via IST in TSS) or a double fault causes a triple fault → CPU reset.

## Loading
`lidt` instruction with pointer to IDT descriptor.

## Related
- [[03-Interrupts/GDT]]
- [[03-Interrupts/Exception-Handlers]]

## Status
> Not started
