use limine::request::MemmapRequest;
use limine::memmap::MEMMAP_USABLE;
use x86_64::structures::paging::{FrameAllocator as X86FrameAllocator, PhysFrame, Size4KiB};
use x86_64::PhysAddr;

// Limine memory map request — filled before _start runs
#[unsafe(link_section = ".requests")]
static MEMORY_MAP: MemmapRequest = MemmapRequest::new();

// Bump-style physical frame allocator.
// Walks usable memory map entries, hands out 4KB-aligned physical frames.
// Cannot free — frames are permanent until kernel restarts.
pub struct FrameAllocator {
    regions: &'static [&'static limine::memmap::Entry],
    current_region: usize,
    current_offset: u64,
}

impl FrameAllocator {
    pub fn new(regions: &'static [&'static limine::memmap::Entry]) -> Self {
        FrameAllocator {
            regions,
            current_region: 0,
            current_offset: 0,
        }
    }

    // Returns the physical address of the next free 4KB frame, or None if OOM.
    pub fn allocate(&mut self) -> Option<u64> {
        // Skip exhausted regions
        if self.current_region >= self.regions.len() {
            return None;
        }

        // Skip non-usable regions (reserved, firmware, kernel, etc.)
        while self.current_region < self.regions.len()
            && self.regions[self.current_region].type_ != MEMMAP_USABLE
        {
            self.current_region += 1;
        }

        if self.current_region >= self.regions.len() {
            return None;
        }

        let address = self.regions[self.current_region].base + self.current_offset;

        // Advance pointer — move to next region if current is exhausted
        self.current_offset += 4096;
        if self.current_offset >= self.regions[self.current_region].length {
            self.current_region += 1;
            self.current_offset = 0;
        }

        Some(address)
    }
}

// Implement x86_64's FrameAllocator trait so map_to() can use our allocator
// for intermediate page table frames.
unsafe impl X86FrameAllocator<Size4KiB> for FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let addr = self.allocate()?;
        Some(PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

pub static mut FRAME_ALLOCATOR: Option<FrameAllocator> = None;

pub fn init() {
    if let Some(response) = MEMORY_MAP.response() {
        unsafe {
            FRAME_ALLOCATOR = Some(FrameAllocator::new(response.entries()));
        }
    }
}
