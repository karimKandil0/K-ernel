use core::alloc::{GlobalAlloc, Layout};
use core::mem::{size_of, align_of};
use crate::sync::mutex::Mutex;

struct FreeBlock {
    size: usize,
    next: *mut FreeBlock,
}

pub struct LinkedListAllocator {
    head: Mutex<*mut FreeBlock>,
}

unsafe impl Send for LinkedListAllocator {}
unsafe impl Sync for LinkedListAllocator {}

#[global_allocator]
pub static ALLOCATOR: LinkedListAllocator = LinkedListAllocator::new();

impl LinkedListAllocator {
    pub const fn new() -> Self {
        LinkedListAllocator {
            head: Mutex::new(core::ptr::null_mut()),
        }
    }

    pub unsafe fn init(&self, heap_start: usize, heap_size: usize) {
        unsafe {
            let block = heap_start as *mut FreeBlock;
            (*block).size = heap_size;
            (*block).next = core::ptr::null_mut();
            *self.head.lock() = block;
        }
    }
}

fn required_size(layout: Layout) -> usize {
    let min = layout.size().max(size_of::<FreeBlock>());
    align_up(min, align_of::<FreeBlock>())
}

unsafe impl GlobalAlloc for LinkedListAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let required = required_size(layout);
        let mut head = self.head.lock();
        let mut current = *head as *mut FreeBlock;
        let mut prev: *mut FreeBlock = core::ptr::null_mut();

        while !current.is_null() {
            if (*current).size >= required {
                let remainder = (*current).size - required;
                if remainder >= size_of::<FreeBlock>() {
                    // Block is big enough to split — carve off the front, leave remainder
                    let new_block = (current as usize + required) as *mut FreeBlock;
                    (*new_block).size = remainder;
                    (*new_block).next = (*current).next;
                    if prev.is_null() {
                        *head = new_block;
                    } else {
                        (*prev).next = new_block;
                    }
                } else {
                    // Remainder too small for a header — give the whole block
                    if prev.is_null() {
                        *head = (*current).next;
                    } else {
                        (*prev).next = (*current).next;
                    }
                }
                return current as *mut u8;
            }
            prev = current;
            current = (*current).next;
        }

        core::ptr::null_mut() // OOM
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = required_size(layout);
        let new_block = ptr as *mut FreeBlock;
        (*new_block).size = size;
        (*new_block).next = core::ptr::null_mut();

        let mut head = self.head.lock();
        let mut current = *head;
        let mut prev: *mut FreeBlock = core::ptr::null_mut();

        // Walk to find insertion point sorted by address
        while !current.is_null() && (current as usize) < (new_block as usize) {
            prev = current;
            current = (*current).next;
        }

        // Insert new_block between prev and current
        (*new_block).next = current;
        if prev.is_null() {
            *head = new_block;
        } else {
            (*prev).next = new_block;
        }

        // Coalesce with next neighbor
        if !(*new_block).next.is_null() {
            let next = (*new_block).next;
            if new_block as usize + (*new_block).size == next as usize {
                (*new_block).size += (*next).size;
                (*new_block).next = (*next).next;
            }
        }

        // Coalesce with prev neighbor
        if !prev.is_null() {
            if prev as usize + (*prev).size == new_block as usize {
                (*prev).size += (*new_block).size;
                (*prev).next = (*new_block).next;
            }
        }
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
