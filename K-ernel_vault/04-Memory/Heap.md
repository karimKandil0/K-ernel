# Heap Allocator

## What is it?
Dynamic memory allocator. Enables `alloc` crate → `Box`, `Vec`, `String`, etc.

## How to enable in no_std
1. Implement `GlobalAlloc` trait
2. Register with `#[global_allocator]`
3. Provide a heap region (virtual address range backed by physical frames)

## Algorithm options
1. **Bump allocator** — allocate only, no free. Simplest possible.
2. **Linked list allocator** — free list of blocks. Can deallocate.
3. **slab allocator** — pools of same-size objects. Fast for fixed-size allocs.

## What we need first
- Paging set up (so we can map virtual → physical for heap region)
- Frame allocator (to get physical memory)

## Related
- [[04-Memory/Frame-Allocator]]
- [[04-Memory/Paging]]

## Status
> Not started
