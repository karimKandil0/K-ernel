# Frame Allocator

## What is it?
Manages physical memory. Hands out 4KB "frames" (physical pages) on request.

## Why separate from heap allocator?
- Frame allocator = physical memory (raw pages)
- Heap allocator = virtual memory (gives you `Box<T>`, `Vec<T>`, etc.)
- Frame allocator is lower level — heap allocator calls it

## Input
Memory map from multiboot2 — tells us which physical regions are usable (not reserved by hardware/firmware).

## Algorithm options
1. **Bump allocator** — just increment a pointer. Simple, can't free. Good start.
2. **Bitmap allocator** — one bit per frame, set/clear on alloc/free.
3. **Free list** — linked list of free frames.

We start with bump.

## Related
- [[04-Memory/Paging]]
- [[04-Memory/Heap]]
- [[01-Boot/Multiboot2]]

## Status
> Not started
