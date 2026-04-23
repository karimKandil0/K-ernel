use crate::memory::paging::HHDM_OFFSET;
use crate::storage::pci::PciDevice;

const HBA_CAP:  usize = 0x00;
const HBA_GHC:  usize = 0x04;
const HBA_IS:   usize = 0x08;
const HBA_PI:   usize = 0x0C;

const PORT_CLB:  usize = 0x00;
const PORT_CLBU: usize = 0x04;
const PORT_FB:   usize = 0x08;
const PORT_FBU:  usize = 0x0C;
const PORT_IS:   usize = 0x10;
const PORT_IE:   usize = 0x14;
const PORT_CMD:  usize = 0x18;
const PORT_TFD:  usize = 0x20;
const PORT_SIG:  usize = 0x24;
const PORT_SSTS: usize = 0x28;
const PORT_SERR: usize = 0x30;
const PORT_CI:   usize = 0x38;

const CMD_ST:  u32 = 1 << 0;
const CMD_FRE: u32 = 1 << 4;
const CMD_FR:  u32 = 1 << 14;
const CMD_CR:  u32 = 1 << 15;

pub struct AhciPort {
    pub port_num: usize,
    pub hba_base: u64,
}

unsafe fn hba_read(base: u64, offset: usize) -> u32 {
    unsafe {
        let ptr = (base + offset as u64) as *const u32;
        ptr.read_volatile()
    }
}

unsafe fn hba_write(base: u64, offset: usize, value: u32) {
    unsafe {
        let ptr = (base + offset as u64) as *mut u32;
        ptr.write_volatile(value);
    }
}

pub unsafe fn port_read(base: u64, port: usize, offset: usize) -> u32 {
    unsafe { hba_read(base, 0x100 + port * 0x80 + offset) }
}

pub unsafe fn port_write(base: u64, port: usize, offset: usize, value: u32) {
    unsafe { hba_write(base, 0x100 + port * 0x80 + offset, value) }
}

unsafe fn stop_port(base: u64, port: usize) {
    unsafe {
        let mut cmd = port_read(base, port, PORT_CMD);
        cmd &= !CMD_ST;
        port_write(base, port, PORT_CMD, cmd);

        loop {
            if port_read(base, port, PORT_CMD) & CMD_CR == 0 {
                break;
            }
        }

        cmd = port_read(base, port, PORT_CMD);
        cmd &= !CMD_FRE;
        port_write(base, port, PORT_CMD, cmd);

        loop {
            if port_read(base, port, PORT_CMD) & CMD_FR == 0 {
                break;
            }
        }
    }
}

unsafe fn start_port(base: u64, port: usize) {
    unsafe {
        loop {
            if port_read(base, port, PORT_CMD) & CMD_CR == 0 {
                break;
            }
        }

        let mut cmd = port_read(base, port, PORT_CMD);
        cmd |= CMD_FRE;
        cmd |= CMD_ST;
        port_write(base, port, PORT_CMD, cmd);
    }
}

unsafe fn init_port(base: u64, port: usize) {
    unsafe {
        stop_port(base, port);

        let clb_phys = {
            if let Some(ref mut fa) = crate::memory::frame_allocator::FRAME_ALLOCATOR {
                fa.allocate().expect("out of frames")
            } else {
                panic!("no frame allocator");
            }
        };

        let fb_phys = {
            if let Some(ref mut fa) = crate::memory::frame_allocator::FRAME_ALLOCATOR {
                fa.allocate().expect("out of frames")
            } else {
                panic!("no frame allocator");
            }
        };

        let hhdm = HHDM_OFFSET;
        let clb_virt = (clb_phys + hhdm) as *mut u8;
        let fb_virt  = (fb_phys  + hhdm) as *mut u8;
        core::ptr::write_bytes(clb_virt, 0, 4096);
        core::ptr::write_bytes(fb_virt,  0, 4096);

        port_write(base, port, PORT_CLB,  (clb_phys & 0xFFFFFFFF) as u32);
        port_write(base, port, PORT_CLBU, (clb_phys >> 32) as u32);
        port_write(base, port, PORT_FB,   (fb_phys  & 0xFFFFFFFF) as u32);
        port_write(base, port, PORT_FBU,  (fb_phys  >> 32) as u32);

        port_write(base, port, PORT_SERR, 0xFFFFFFFF);
        port_write(base, port, PORT_IS,   0xFFFFFFFF);

        start_port(base, port);
    }
}

pub static mut AHCI_PORT: Option<AhciPort> = None;

pub unsafe fn find_and_init(dev: &PciDevice) {
    unsafe {
        let hba_base = (dev.bar5 as u64 & !0xF) + HHDM_OFFSET;
        let pi = hba_read(hba_base, HBA_PI);

        for port in 0..32 {
            if pi & (1 << port) == 0 {
                continue;
            }

            let ssts = port_read(hba_base, port, PORT_SSTS);
            let det = ssts & 0x0F;
            if det != 3 {
                continue;
            }

            let sig = port_read(hba_base, port, PORT_SIG);
            if sig != 0x00000101 {
                continue;
            }

            init_port(hba_base, port);

            AHCI_PORT = Some(AhciPort { port_num: port, hba_base });
            crate::print!("\nahci: port {} ready\n", port);
            return;
        }

        crate::print!("\nahci: no SATA drive found\n");
    }
}
