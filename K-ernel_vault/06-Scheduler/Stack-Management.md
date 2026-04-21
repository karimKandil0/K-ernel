# Stack Management

## Stack per task
Each task needs its own stack. On switch, RSP changes → each task runs on its own stack.

## Stack layout at task creation
Must be set up to look like the task was interrupted:
```
[top of stack]
ss
rsp
rflags
cs
rip   ← initial entry point
```
So first `iretq` "returns" into the new task.

## Stack size
Typically 4KB–64KB per task. Must be page-aligned.

## Guard pages
Unmapped page below stack. Stack overflow → page fault (caught) instead of silent corruption.

## Kernel stack vs user stack
For now: kernel-only tasks. Both stacks same privilege level.

## Related
- [[06-Scheduler/Context-Switching]]
- [[04-Memory/Paging]]

## Status
> Not started
