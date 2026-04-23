use limine::request::HhdmRequest;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, Size4KiB};
use x86_64::VirtAddr;
use crate::memory::frame_allocator::FRAME_ALLOCATOR;

// Limine HHDM request — Limine maps all physical RAM at a fixed virtual offset.
// We store this offset to convert physical addresses → virtual for page table access.
#[unsafe(link_section = ".requests")]
static HHDM: HhdmRequest = HhdmRequest::new();

// Virtual offset where all physical memory is mapped by Limine.
// Access physical address P at virtual address: P + HHDM_OFFSET
pub static mut HHDM_OFFSET: u64 = 0;

// Heap virtual address range — 1MB mapped during init
pub const HEAP_START: u64 = 0xFFFF_C000_0000_0000;
pub const HEAP_SIZE: u64 = 1024 * 1024; // 1MB

pub fn init() {
    if let Some(response) = HHDM.response() {
        unsafe {
            HHDM_OFFSET = response.offset;
        }

        // Read CR3 to get physical address of current PML4 table (set by Limine)
        let (frame, _) = Cr3::read();
        let phys = frame.start_address();

        // Access PML4 via HHDM — page tables live at physical addresses,
        // but we must dereference through the virtual HHDM mapping
        let virt = VirtAddr::new(unsafe { HHDM_OFFSET } + phys.as_u64());
        let pml4 = unsafe { &mut *(virt.as_u64() as *mut PageTable) };

        // OffsetPageTable handles the 4-level walk automatically using HHDM offset
        let mut mapper = unsafe { OffsetPageTable::new(pml4, VirtAddr::new(HHDM_OFFSET)) };

        // Map the heap region so BumpAllocator has physical memory backing it
        unsafe {
            if let Some(ref mut allocator) = FRAME_ALLOCATOR {
                map_heap(&mut mapper, allocator);
                crate::memory::heap::ALLOCATOR.init(HEAP_START as usize, HEAP_SIZE as usize);
            }
        }
    }
}

// Maps HEAP_SIZE bytes at HEAP_START, one 4KB page at a time.
// Each page gets a physical frame from the frame allocator.
pub fn map_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl x86_64::structures::paging::FrameAllocator<Size4KiB>,
) {
    let start = Page::containing_address(VirtAddr::new(HEAP_START));
    let end = Page::containing_address(VirtAddr::new(HEAP_START + HEAP_SIZE - 1));

    for page in Page::range_inclusive(start, end) {
        let frame = frame_allocator.allocate_frame().expect("out of frames");
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper
                .map_to(page, frame, flags, frame_allocator)
                .expect("map_to failed")
                .flush();
        }
    }
}
