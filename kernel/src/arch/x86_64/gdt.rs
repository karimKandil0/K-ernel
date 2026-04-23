use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;
use core::ptr::addr_of;

// IST slot used for the double fault handler's alternate stack
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// Dedicated stack for double fault handler.
// x86 stacks grow downward — TSS gets pointer to top of this buffer.
// Must not be `static mut` — we access it read-only via the TSS pointer.
static DOUBLE_FAULT_STACK: [u8; 4096 * 5] = [0; 4096 * 5]; // 20KB

static mut TSS: TaskStateSegment = TaskStateSegment::new();
static mut GDT: Option<GlobalDescriptorTable> = None;

// Initialize and load our own GDT, replacing Limine's temporary one.
// Must be called before IDT is loaded.
pub fn init() {
    unsafe {
        // Point TSS IST slot 0 at the top of the double fault stack
        let stack_top = VirtAddr::from_ptr(DOUBLE_FAULT_STACK.as_ptr())
            + DOUBLE_FAULT_STACK.len() as u64;
        TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = stack_top;

        // Build GDT with required segments:
        // - null descriptor (index 0, required by spec)
        // - kernel code segment (ring 0, executable)
        // - kernel data segment (ring 0, read/write)
        // - TSS descriptor (holds IST stack pointers)
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.append(Descriptor::kernel_code_segment());
        let data_selector = gdt.append(Descriptor::kernel_data_segment());
        let tss_selector = gdt.append(Descriptor::tss_segment(&*addr_of!(TSS)));

        GDT = Some(gdt);
        if let Some(ref g) = GDT {
            g.load();
        }

        // Reload segment registers — still pointing at Limine's old GDT entries
        use x86_64::instructions::segmentation::{CS, DS, SS, Segment};
        use x86_64::instructions::tables::load_tss;

        CS::set_reg(code_selector);
        DS::set_reg(data_selector);
        SS::set_reg(data_selector);
        load_tss(tss_selector);
    }
}
