use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicU64, Ordering};
use crate::paging::{HEAP_START, HEAP_SIZE};

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new(HEAP_START, HEAP_START + HEAP_SIZE);


pub struct BumpAllocator {
    start: u64,
    end: u64,
    next: AtomicU64
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alloc_start = align_up(self.next.load(Ordering::Relaxed), layout.align() as u64);
        let alloc_end = alloc_start + layout.size() as u64;

        if alloc_end > self.end {
            core::ptr::null_mut()
        } else {
            self.next.store(alloc_end, Ordering::Relaxed);
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {

    }
}

fn align_up(addr: u64, align: u64) -> u64 {
    (addr + align -1) & !(align - 1)
}

impl BumpAllocator {
    pub const fn new(start: u64, end: u64) -> Self {
        BumpAllocator { start, end, next: AtomicU64::new(start) }
    }
}
