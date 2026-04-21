use limine::request::MemmapRequest;
use limine::memmap::MEMMAP_USABLE;
use crate::print;
use crate::WRITER;

#[unsafe(link_section = ".requests")]
static MEMORY_MAP: MemmapRequest = MemmapRequest::new();

struct FrameAllocator {
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
