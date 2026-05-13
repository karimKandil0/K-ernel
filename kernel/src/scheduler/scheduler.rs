extern crate alloc;
use crate::sync::mutex::Mutex;
use alloc::boxed::Box;

pub const STACK_SIZE: usize = 4096 * 4;
pub const MAX_TASKS: usize = 8;

pub struct Task {
    pub id: usize,
    pub rsp: u64,
    pub stack: Box<[u8]>,
}

struct Scheduler {
    tasks: [Option<Task>; MAX_TASKS],
    current: usize,
    count: usize,
}

impl Scheduler {
    const fn new() -> Self {
        Scheduler {
            tasks: [None, None, None, None, None, None, None, None],
            current: 0,
            count: 0,
        }
    }
}

unsafe impl Sync for Scheduler {}

pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
