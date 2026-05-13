use x86_64::instructions::port::Port;

pub fn init() {
    let divisor: u16 = 11931;
    unsafe {
        let mut cmd: Port<u8> = Port::new(0x43);
        let mut data: Port<u8> = Port::new(0x40);
        cmd.write(0x36);
        data.write((divisor & 0xFF) as u8);
        data.write((divisor >> 8) as u8);
    }
}
