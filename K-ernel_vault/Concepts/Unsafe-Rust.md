# Unsafe Rust

## What it is
`unsafe { }` block tells compiler: "I know what I'm doing, skip your checks here."

## What unsafe enables
- Dereference raw pointers (`*const T`, `*mut T`)
- Call unsafe functions
- Access/modify static mutable variables
- Implement unsafe traits
- Use inline assembly

## What it does NOT disable
- Borrow checker (outside the unsafe block)
- Type system
- Lifetimes

## Why kernel code needs it
- Raw hardware access (VGA buffer, port I/O) = raw pointers
- Manual memory management = raw pointers
- Interrupt handlers = assembly-adjacent ABI

## Rule
Unsafe blocks should be as small as possible. Wrap them in safe abstractions. The goal: push `unsafe` to the edges, keep the rest safe.

## Related
- [[02-VGA/Memory-Mapped-IO]]
