# Multiboot2

> [!warning] SUPERSEDED
> We switched to Limine after session 01. Reason: Limine boots directly into 64-bit long mode — no assembly trampoline needed. Multiboot2 drops you in 32-bit protected mode and you have to write the 32→64 transition yourself.
> See [[01-Boot/Limine]] for the current approach.

## What is it?
A bootloader protocol. Lets GRUB (or QEMU direct) load our kernel and pass info about memory, modules, etc.

## Why we moved away
- Leaves CPU in 32-bit protected mode — required assembly to enter 64-bit long mode before Rust code can run
- More setup, more error-prone
- Limine gives a cleaner, more modern protocol with no assembly trampoline

## Historical note
We originally planned to use multiboot2 based on tutorial prevalence. Switched to Limine after weighing the tradeoffs.
