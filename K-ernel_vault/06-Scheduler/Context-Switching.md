# Context Switching

## What is it?
Saving one task's CPU state and restoring another's. The core of preemptive multitasking.

## What "state" means
- General purpose registers (RAX, RBX, ... R15)
- RIP (instruction pointer)
- RSP (stack pointer)
- RFLAGS
- CR3 (page table root, if tasks have separate address spaces)

## How it works
1. Timer interrupt fires
2. Kernel saves current task's registers to its stack/TCB
3. Kernel picks next task
4. Kernel restores next task's registers
5. `iretq` — returns into next task

## Task Control Block (TCB)
Struct holding saved state for a task:
```
rsp: u64  // saved stack pointer (all other regs saved on that stack)
```

## Related
- [[06-Scheduler/Stack-Management]]
- [[03-Interrupts/IDT]]

## Status
> Not started
