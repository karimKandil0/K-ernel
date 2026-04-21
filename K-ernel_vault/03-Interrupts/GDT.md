# Global Descriptor Table (GDT)

## What is it?
A table that defines memory segments for the CPU. x86_64 uses it mostly for privilege levels (ring 0 vs ring 3) and the TSS.

## Why we need it
CPU requires a valid GDT loaded at all times. For a kernel, we need at minimum:
- Null descriptor (required)
- Kernel code segment
- Kernel data segment
- TSS descriptor (for interrupt stack switching)

## Task State Segment (TSS)
Holds stack pointers for privilege-level switches. When an interrupt fires in user mode, CPU needs to know where the kernel stack is.

## How to load it
`lgdt` instruction with a pointer to a `GDT descriptor` struct (base address + limit).

## Related
- [[03-Interrupts/IDT]]
- [[Concepts/x86-Privilege-Levels]]

## Status
> Not started
