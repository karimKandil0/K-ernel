# x86 Privilege Levels (Rings)

## What they are
x86 has 4 privilege levels (rings 0–3). We use 2:
- Ring 0 — kernel mode (full hardware access)
- Ring 3 — user mode (restricted)

## How enforced
- CPU tracks current privilege level in CS register (CPL field)
- Certain instructions only valid in ring 0 (`lgdt`, `lidt`, `cli`, `hlt`, etc.)
- Page table entries have a "user" bit — if clear, ring 3 can't access

## Switching rings
- Ring 3 → Ring 0: `syscall` instruction or interrupt
- Ring 0 → Ring 3: `iretq` with user-mode CS/SS

## Why it matters now
For our kernel-only implementation: everything runs in ring 0. No user mode yet.
GDT still needs correct segment descriptors with ring 0 privilege.

## Related
- [[03-Interrupts/GDT]]
