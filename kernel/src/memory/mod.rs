pub mod frame_allocator;
pub mod heap;
pub mod paging;

pub use frame_allocator::{FrameAllocator, FRAME_ALLOCATOR};
pub use paging::{HHDM_OFFSET, HEAP_START, HEAP_SIZE};
