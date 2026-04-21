# Virtual vs Physical Memory

## Physical memory
Actual RAM. Addresses are hardware addresses. `0x00000000` to however much RAM you have.

## Virtual memory
What programs (and the kernel) see. CPU translates virtual → physical via page tables.

## Why bother?
- Isolation: process A can't see process B's memory (different page tables)
- Flexibility: non-contiguous physical RAM can appear contiguous virtually
- Protection: mark pages read-only, no-execute, etc.

## Who manages the mapping?
The kernel. Page tables are kernel data structures. Hardware (MMU) does the translation.

## TLB (Translation Lookaside Buffer)
Cache for page table lookups. When you change a mapping, must flush TLB (`invlpg` instruction).

## Related
- [[04-Memory/Paging]]
- [[04-Memory/Frame-Allocator]]
