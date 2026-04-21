# Limine Bootloader

## What is it?
Modern, actively maintained bootloader protocol. We switched to this from multiboot2.

## Why Limine over multiboot2
- Boots kernel directly in 64-bit long mode — Rust entry point runs in x86_64 immediately
- No assembly trampoline needed to switch from 32-bit protected mode
- Typed protocol structs (clean Rust interface via `limine` crate)
- Better framebuffer support (pixel-based, not limited to VGA text mode)
- Used by serious hobby OSes

## How it works
1. Write a `limine.conf` config file — tells Limine which kernel ELF to load
2. Limine loads kernel, sets up paging, enters long mode
3. Jumps to our `_start` — we're already in 64-bit Rust
4. Limine passes a `BootInfo` struct pointer via a request/response protocol

## Request/response protocol
Kernel declares static `Request` structs (from `limine` crate). Limine finds them in the binary, fills in the response pointers before jumping to entry.

Example requests:
- `MemoryMapRequest` — usable physical memory regions
- `HHDMRequest` — higher-half direct map offset
- `FramebufferRequest` — framebuffer info
- `KernelAddressRequest` — where kernel was loaded

## What we need
- `limine` crate in `Cargo.toml`
- `limine.conf` config file
- ISO structure: kernel ELF + Limine files + config
- `xorriso` to build the ISO

## Key concepts
- [[Concepts/ELF-Binary]] — kernel is an ELF file Limine loads
- [[01-Boot/Linker-Script]] — still needed to control kernel memory layout

## Related
- [[01-Boot/Linker-Script]]
- [[00-Overview/Architecture]]

## Status
> Not started
