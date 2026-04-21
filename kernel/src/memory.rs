use limine::request::MemmapRequest;
use limine::memmap::MEMMAP_USABLE;
use crate::print;
use crate::WRITER;

#[unsafe(link_section = ".requests")]
static MEMORY_MAP: MemmapRequest = MemmapRequest::new();

pub fn init() {
    if let Some(response) = MEMORY_MAP.response() {
        for entry in response.entries() {
            if entry.type_ == MEMMAP_USABLE {
                print!("base: {}, length: {}\n", entry.base, entry.length);
            }
        }
    }
}
