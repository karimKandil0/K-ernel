use x86_64::instructions::port::Port;

pub fn dispatch(cmd: &str) {
    match cmd {
        "help" => crate::print!("\ncommands: help, clear, echo\n"),
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
        _ if cmd.starts_with("echo ") => crate::print!("\n{}\n", &cmd[5..]),
        _ => crate::print!("\nunknown command: {}\n", cmd),
    }
}
