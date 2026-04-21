# Multiboot2

## What is it?
A bootloader protocol. Lets GRUB (or QEMU direct) load our kernel and pass info about memory, modules, etc.

## How it works
Kernel binary must contain a multiboot2 header at a specific offset. Bootloader finds it, loads kernel into memory, jumps to entry point.

## What we need to implement
- Multiboot2 header in assembly (magic number, architecture, checksum)
- Entry point symbol `_start`
- Read multiboot2 info struct passed in `ebx`

## Key concepts
- [[Concepts/ELF-Binary]] — kernel is an ELF file
- [[01-Boot/Linker-Script]] — controls where header lands in binary

## Status
> Not started
