use x86_64::instructions::port::Port;

fn pci_read_u32(bus: u8, device: u8, func: u8, offset: u8) -> u32 {
    let address: u32 = 0x80000000
        | ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((func as u32) << 8)
        | ((offset as u32) & 0xFC);

    unsafe {
        let mut addr_port: Port<u32> = Port::new(0xCF8);
        let mut data_port: Port<u32> = Port::new(0xCFC);
        addr_port.write(address);
        data_port.read()
    }
}

fn pci_write_u32(bus: u8, device: u8, func: u8, offset: u8, value: u32) {
    let address: u32 = 0x80000000
        | ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((func as u32) << 8)
        | ((offset as u32) & 0xFC);

    unsafe {
        let mut addr_port: Port<u32> = Port::new(0xCF8);
        let mut data_port: Port<u32> = Port::new(0xCFC);
        addr_port.write(address);
        data_port.write(value);
    }
}

pub struct PciDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub bar5: u32,
}

pub fn find_ahci() -> Option<PciDevice> {
    for bus in 0u8..=255 {
        for device in 0u8..32 {
            let id = pci_read_u32(bus, device, 0, 0x00);
            let vendor_id = (id & 0xFFFF) as u16;

            if vendor_id == 0xFFFF {
                continue;
            }

            let device_id = ((id >> 16) & 0xFFFF) as u16;

            let class_reg = pci_read_u32(bus, device, 0, 0x08);
            let class    = ((class_reg >> 24) & 0xFF) as u8;
            let subclass = ((class_reg >> 16) & 0xFF) as u8;

            if class == 0x01 && subclass == 0x06 {
                let cmd = pci_read_u32(bus, device, 0, 0x04);
                pci_write_u32(bus, device, 0, 0x04, cmd | 0x4);

                let bar5 = pci_read_u32(bus, device, 0, 0x24);

                return Some(PciDevice { bus, device, function: 0, vendor_id, device_id, bar5 });
            }
        }
    }
    None
}
