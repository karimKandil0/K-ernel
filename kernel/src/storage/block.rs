use crate::storage::ahci::AHCI_PORT;
use crate::memory::paging::HHDM_OFFSET;
use crate::memory::frame_allocator::FRAME_ALLOCATOR;

#[repr(C)]
struct HbaCmdHeader {
    flags: u16,
    prdtl: u16,
    prdbc: u32,
    ctba:  u32,
    ctbau: u32,
    _reserved: [u32; 4],
}

#[repr(C)]
struct HbaPrd {
    dba:       u32,
    dbau:      u32,
    _reserved: u32,
    dbc:       u32,
}

#[repr(C)]
struct HbaCmdTable {
    cfis:      [u8; 64],
    acmd:      [u8; 16],
    _reserved: [u8; 48],
    prdt:      [HbaPrd; 1],
}

pub enum BlockError {
    NoPort,
    DeviceBusy,
    DmaError,
}

pub unsafe fn read_sector(sector: u64, buf: &mut [u8; 512]) -> Result<(), BlockError> {
    unsafe {
        let port = match &raw const AHCI_PORT {
            p => match &*p {
                Some(p) => p,
                None => return Err(BlockError::NoPort),
            }
        };

        let port_num = port.port_num;
        let hba_base = port.hba_base;

        let ct_phys = {
            if let Some(ref mut fa) = FRAME_ALLOCATOR {
                fa.allocate().expect("out of frames")
            } else {
                return Err(BlockError::NoPort);
            }
        };

        let data_phys = {
            if let Some(ref mut fa) = FRAME_ALLOCATOR {
                fa.allocate().expect("out of frames")
            } else {
                return Err(BlockError::NoPort);
            }
        };

        let hhdm = HHDM_OFFSET;

        let ct_virt = (ct_phys + hhdm) as *mut HbaCmdTable;
        core::ptr::write_bytes(ct_virt as *mut u8, 0, 4096);

        let cfis = &mut (*ct_virt).cfis;
        cfis[0]  = 0x27;
        cfis[1]  = 0x80;
        cfis[2]  = 0x25;
        cfis[3]  = 0x00;
        cfis[4]  = (sector & 0xFF) as u8;
        cfis[5]  = ((sector >> 8)  & 0xFF) as u8;
        cfis[6]  = ((sector >> 16) & 0xFF) as u8;
        cfis[7]  = 0x40;
        cfis[8]  = ((sector >> 24) & 0xFF) as u8;
        cfis[9]  = ((sector >> 32) & 0xFF) as u8;
        cfis[10] = ((sector >> 40) & 0xFF) as u8;
        cfis[11] = 0x00;
        cfis[12] = 1;
        cfis[13] = 0;

        (*ct_virt).prdt[0].dba  = (data_phys & 0xFFFFFFFF) as u32;
        (*ct_virt).prdt[0].dbau = (data_phys >> 32) as u32;
        (*ct_virt).prdt[0].dbc  = 511;

        let clb_phys = {
            let lo = crate::storage::ahci::port_read(hba_base, port_num, 0x00) as u64;
            let hi = crate::storage::ahci::port_read(hba_base, port_num, 0x04) as u64;
            lo | (hi << 32)
        };
        let clb_virt = (clb_phys + hhdm) as *mut HbaCmdHeader;

        (*clb_virt).flags = 5;
        (*clb_virt).prdtl = 1;
        (*clb_virt).prdbc = 0;
        (*clb_virt).ctba  = (ct_phys & 0xFFFFFFFF) as u32;
        (*clb_virt).ctbau = (ct_phys >> 32) as u32;

        let mut timeout = 1_000_000u32;
        while crate::storage::ahci::port_read(hba_base, port_num, 0x20) & 0x88 != 0 {
            timeout -= 1;
            if timeout == 0 {
                return Err(BlockError::DeviceBusy);
            }
        }

        crate::storage::ahci::port_write(hba_base, port_num, 0x38, 1);

        timeout = 1_000_000u32;
        loop {
            let ci = crate::storage::ahci::port_read(hba_base, port_num, 0x38);
            if ci & 1 == 0 {
                break;
            }
            let tfd = crate::storage::ahci::port_read(hba_base, port_num, 0x20);
            if tfd & 0x01 != 0 {
                return Err(BlockError::DmaError);
            }
            timeout -= 1;
            if timeout == 0 {
                return Err(BlockError::DeviceBusy);
            }
        }

        let data_virt = (data_phys + hhdm) as *const u8;
        core::ptr::copy_nonoverlapping(data_virt, buf.as_mut_ptr(), 512);

        Ok(())
    }
}
