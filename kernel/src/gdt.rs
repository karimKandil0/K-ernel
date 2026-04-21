use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;
use core::ptr::addr_of;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

static DOUBLE_FAULT_STACK: [u8; 4096 * 5] = [0; 4096 * 5];

static mut TSS: TaskStateSegment = TaskStateSegment::new();

static mut GDT: Option<GlobalDescriptorTable> = None;

pub fn init() {
    unsafe {
        let stack_top = VirtAddr::from_ptr(DOUBLE_FAULT_STACK.as_ptr()) + DOUBLE_FAULT_STACK.len() as u64;
        TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = stack_top;

        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.append(Descriptor::kernel_code_segment());
        let data_selector = gdt.append(Descriptor::kernel_data_segment());
        let tss_selector = gdt.append(Descriptor::tss_segment(&*addr_of!(TSS)));
        GDT = Some(gdt); 
        if let Some(ref g) = GDT {
            g.load();
        }

        use x86_64::instructions::segmentation::{CS, DS, SS, Segment};
        use x86_64::instructions::tables::load_tss;

        CS::set_reg(code_selector);
        DS::set_reg(data_selector);
        SS::set_reg(data_selector);
        load_tss(tss_selector);
    }
}
