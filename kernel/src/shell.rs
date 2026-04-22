use alloc::vec::Vec;
use crate::print;
use crate::WRITER;

pub struct Shell {
    buffer: Vec<u8>,
}

impl Shell {
    pub fn new() -> Self {
        Shell { buffer: Vec::new() }
    }

    pub fn push(&mut self, c: u8) {
        if c == 0x08 {
            if self.buffer.pop().is_some() {
                unsafe {
                    if let Some(ref mut w) = crate::WRITER {
                        w.backspace();
                    }
                }
            }
        } else {
            self.buffer.push(c);
            print!("{}", c as char);
        }
    }


    pub fn execute(&mut self) {
        let cmd = core::str::from_utf8(&self.buffer).unwrap_or("");
        let cmd = cmd.trim();

        match cmd {
            "help" => print!("\ncommand: help, clear, echo\n"),
            "clear" => { /* clear screen - todo */},
            _ if cmd.starts_with("echo ") => print!("\n{}\n", &cmd[5..]),
            _ => print!("\nunknown command: {}\n", cmd),
        }

        self.buffer.clear();
        print!("\n> ")
    }

}

pub fn handle_char(c: u8) {
    unsafe {
        if let Some(ref mut shell) = SHELL {
            if c == b'\n' {
                shell.execute();
            } else {
                shell.push(c);
            }
        }
    }
}

pub static mut SHELL: Option<Shell> = None;

pub fn init() {
    unsafe {
        SHELL = Some(Shell::new());
    }

    print!("> ");
}


