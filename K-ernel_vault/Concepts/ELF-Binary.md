# ELF Binary Format

## What is it?
Executable and Linkable Format. Standard binary format on Linux/Unix. Our kernel is an ELF file.

## Structure
- ELF header — magic bytes, architecture, entry point address
- Program headers — segments (how to load into memory)
- Section headers — sections (`.text`, `.data`, `.bss`, etc.)

## Why it matters for kernel dev
- Bootloader reads ELF to know where to load kernel and where to jump
- Linker script controls how ELF sections are arranged
- `objdump -x kernel.elf` shows you exactly what's in your binary

## Key fields in ELF header
- `e_entry` — entry point virtual address (`_start`)
- `e_phoff` — offset to program headers
- `e_machine` — `0x3E` = x86_64

## Related
- [[01-Boot/Linker-Script]]
- [[01-Boot/Multiboot2]]
