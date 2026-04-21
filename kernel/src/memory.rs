use limine::request::MemmapRequest;
use limine::memmap::MEMMAP_USABLE;
use x86_64::structures::paging::{FrameAllocator as X86FrameAllocator, PhysFrame, Size4KiB};
use x86_64::PhysAddr;
use crate::print;
use crate::WRITER;

#[unsafe(link_section = ".requests")]
static MEMORY_MAP: MemmapRequest = MemmapRequest::new();

pub struct FrameAllocator {
    regions: &'static [&'static limine::memmap::Entry],
    current_region: usize,
    current_offset: u64
}

impl FrameAllocator {
    pub fn new(regions: &'static [&'static limine::memmap::Entry]) -> Self {
        FrameAllocator {
            regions,
            current_region: 0,
            current_offset: 0,
        }
    }

    pub fn allocate(&mut self) -> Option<u64> {
        if self.current_region >= self.regions.len() {
            return None
        }

        while self.current_region < self.regions.len() && self.regions[self.current_region].type_ != MEMMAP_USABLE {
            self.current_region += 1;
        }
        
        if self.current_region >= self.regions.len() {
            return None;
        }

        let address: u64 = self.regions[self.current_region].base + self.current_offset;
        
        self.current_offset += 4096;
        if self.current_offset >= self.regions[self.current_region].length {
            self.current_region += 1;
            self.current_offset = 0
        }
        Some(address)
    }
}

unsafe impl X86FrameAllocator<Size4KiB> for FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let addr = self.allocate()?;
        Some(PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

pub static mut FRAME_ALLOCATOR: Option<FrameAllocator> = None;

pub fn init() {
    if let Some(response) = MEMORY_MAP.response() {
        for entry in response.entries() {
            if entry.type_ == MEMMAP_USABLE {
                print!("base: {}, length: {}\n", entry.base, entry.length);
                
            }
        }
        unsafe {
            FRAME_ALLOCATOR = Some(FrameAllocator::new(response.entries()));
        }
    }
}
