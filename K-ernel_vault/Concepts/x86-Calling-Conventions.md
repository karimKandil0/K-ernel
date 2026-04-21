# x86 Calling Conventions

## System V AMD64 ABI (Linux/standard)
What Rust uses by default for `extern "C"` on x86_64.

**Arguments (in order):** RDI, RSI, RDX, RCX, R8, R9, then stack
**Return value:** RAX (+ RDX for 128-bit)
**Caller-saved:** RAX, RCX, RDX, RSI, RDI, R8, R9, R10, R11
**Callee-saved:** RBX, RBP, R12–R15

## x86-interrupt ABI
Special Rust ABI for interrupt handlers. Compiler knows:
- CPU pushed interrupt frame on stack
- Must use `iretq` to return (not `ret`)
- Must preserve all registers

Enabled with: `#![feature(abi_x86_interrupt)]` (nightly only)

## Why this matters
Interrupt handlers have different calling convention than normal functions. Getting it wrong = corrupted registers = undefined behavior / crash.

## Related
- [[03-Interrupts/Exception-Handlers]]
