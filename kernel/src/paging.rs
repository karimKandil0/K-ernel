use limine::request::HhdmRequest;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{OffsetPageTable, PageTable};
use x86_64::structures::paging::{Page, PageTableFlags, Size4KiB, Mapper};
use x86_64::VirtAddr;
use crate::print;
use crate::WRITER;
use crate::memory::FRAME_ALLOCATOR;

#[unsafe(link_section = ".requests")]
static HHDM: HhdmRequest = HhdmRequest::new();

pub static mut HHDM_OFFSET: u64 = 0;
pub const HEAP_START: u64 = 0xFFFF_C000_0000_0000;
pub const HEAP_SIZE: u64 = 1024 * 1024;

pub fn init() {
    if let Some(response) = HHDM.response() {
        print!("HHDM offset: {:#x}\n", response.offset);

        unsafe {
            HHDM_OFFSET = response.offset;
        }

        let (frame, _) = Cr3::read();
        print!("PML4 at: {:#x}\n", frame.start_address().as_u64());

        let phys = frame.start_address();

        let virt = VirtAddr::new(unsafe { HHDM_OFFSET } + phys.as_u64());
        let pml4 = unsafe { &mut *(virt.as_u64() as *mut PageTable) };

        let mut mapper = unsafe { OffsetPageTable::new(pml4, VirtAddr::new(HHDM_OFFSET)) };

        unsafe {
            if let Some(ref mut allocator) = FRAME_ALLOCATOR {
                map_heap(&mut mapper, allocator);
            }
       }

    }

}

pub fn map_heap(mapper: &mut impl Mapper<Size4KiB>, frame_allocator: &mut impl x86_64::structures::paging::FrameAllocator<Size4KiB>) {
    use x86_64::VirtAddr;

    let start = Page::containing_address(VirtAddr::new(HEAP_START));
    let end = Page::containing_address(VirtAddr::new(HEAP_START + HEAP_SIZE -1));

    for page in Page::range_inclusive(start, end) {
        let frame = frame_allocator.allocate_frame().expect("out of frames");
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator).expect("map_to failed").flush();
        }
    }

}
