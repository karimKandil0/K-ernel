# Exception Handlers

## What they are
Rust functions registered in the IDT. Called by CPU when exceptions fire.

## Calling convention
CPU pushes an interrupt stack frame before calling handler:
```
SS
RSP
RFLAGS
CS
RIP
[error code] (for some exceptions)
```

## Handler signature
Must match exactly what CPU expects. The `x86_64` crate provides `InterruptDescriptorTable` and handler type aliases.

Manual (without crate):
```rust
extern "x86-interrupt" fn handler(frame: &InterruptStackFrame) { ... }
```
Requires nightly + `abi_x86_interrupt` feature.

## What each handler should do (minimum)
- Print what happened
- Halt (for now — no recovery yet)

## Related
- [[03-Interrupts/IDT]]
- [[Concepts/x86-Calling-Conventions]]

## Status
> Not started
