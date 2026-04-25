use x86_64::instructions::port::Port;

pub fn dispatch(cmd: &str) {
    match cmd {
        "help" => crate::print!("\ncommands: help, clear, echo, reboot, diskread, ls, cat\n"),
        "clear" => {
            unsafe {
                if let Some(ref mut w) = crate::WRITER {
                    w.clear();
                }
            }
        },
        "reboot" => {
            unsafe {
                Port::<u8>::new(0x64).write(0xFE);
            }
        }
        "diskread" => {
            let mut buf = [0u8; 512];
            unsafe {
                match crate::storage::block::read_sector(0, &mut buf) {
                    Ok(()) => {
                        crate::print!("\nsector 0:");
                        for i in 0..32 {
                            if i % 16 == 0 { crate::print!("\n  "); }
                            crate::print!("{:02x}", buf[i]);
                        }
                        crate::print!("\n");
                    }
                    Err(crate::storage::block::BlockError::NoPort)      => crate::print!("\ndiskread: no port\n"),
                    Err(crate::storage::block::BlockError::DeviceBusy)  => crate::print!("\ndiskread: timeout\n"),
                    Err(crate::storage::block::BlockError::DmaError)    => crate::print!("\ndiskread: DMA error\n"),
                }
            }
        }
        "ls" => unsafe { crate::storage::fat::ls(); },
        _ if cmd.starts_with("cat ") => unsafe { crate::storage::fat::cat(&cmd[4..]); },
        _ if cmd.starts_with("echo ") => crate::print!("\n{}\n", &cmd[5..]),
        _ => crate::print!("\nunknown command: {}\n", cmd),
    }
}
