# Linker Script

## What is it?
A `.ld` file that tells the linker how to arrange sections in the output binary.

## Why we need it
Default linker assumes an OS. We have none. We must manually define:
- Where `.text` (code) lives
- Where `.rodata`, `.data`, `.bss` live
- Entry point symbol

## Key sections
- `.text` — executable code
- `.rodata` — read-only data (string literals, etc.)
- `.data` — initialized global data
- `.bss` — zero-initialized globals (not stored in binary, zeroed at boot)
- `.multiboot2` — our multiboot header (must be within first 32KB)

## Status
> Not started
