# Architecture

High-level view of the kernel's structure. Updated as we build.

## Boot flow
```
QEMU → Limine bootloader → kernel entry (_start) → Rust main
```

## Memory layout
> Fill in as we implement

## Component map
- [[01-Boot/Limine]] — how we get control from the bootloader
- [[02-VGA/VGA-Buffer]] — text output
- [[03-Interrupts/IDT]] — interrupt descriptor table
- [[04-Memory/Paging]] — virtual memory
- [[06-Scheduler/Context-Switching]] — task switching
