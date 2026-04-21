# Paging

## What is it?
Virtual memory mechanism. CPU translates virtual addresses → physical via page tables.

## x86_64 page table structure
4-level hierarchy:
```
CR3 → PML4 → PDPT → PD → PT → physical page
```
Each level is a 512-entry table. Each entry is 8 bytes.

## Page size
Default: 4KB (4096 bytes). Also 2MB (huge pages) and 1GB.

## Why it matters for a kernel
- Kernel lives at high virtual address (e.g. `0xFFFFFFFF80000000`)
- Physical memory may be at different addresses
- Must set up page tables before enabling paging (or use identity mapping initially)

## Identity mapping
Virtual address == physical address. Simple to set up, used early in boot.

## Related
- [[04-Memory/Frame-Allocator]]
- [[Concepts/Virtual-vs-Physical-Memory]]

## Status
> Not started
