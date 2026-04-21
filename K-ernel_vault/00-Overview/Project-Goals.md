# Project Goals

Building a kernel in Rust from scratch. Goal is understanding, not production use.

## What we're building
- Bare metal x86_64 kernel
- Boots via QEMU + Limine
- VGA text output
- Interrupt handling
- Memory management (paging + heap)
- Keyboard input
- Basic scheduler
- Simple shell

## Rules
- No copying code blindly — understand every line before writing it
- Document every decision in [[Journal]]
- Link concepts when they appear

## Target
- Architecture: x86_64
- Bootloader: Limine
- Emulator: QEMU
- Language: Rust (no_std)
