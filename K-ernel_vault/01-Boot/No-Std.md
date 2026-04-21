# no_std in Rust

## What it means
`#![no_std]` tells Rust: don't link the standard library. We only get `core` (no heap, no OS primitives).

## Why
The standard library assumes an OS underneath (file I/O, threads, allocator, etc.). We ARE the OS — nothing underneath us.

## What we lose
- `std::vec`, `std::string`, `std::io`, etc.
- Default panic handler
- Default allocator

## What we keep
- `core` crate — iterators, Option, Result, primitives, math
- `alloc` crate (later, once we have a heap allocator)

## What we must provide
- `#[panic_handler]` — what happens on panic
- `#[no_mangle] pub extern "C" fn _start()` — entry point (no Rust runtime to call main)

## Status
> Not started
