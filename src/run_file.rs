use std::io;
use std::io::Write;
use crate::shell_functions::{get_args, execute_commands};

pub fn run() {
    loop {
        print!("KevShell$> ");
        io::stdout().flush().unwrap();

        let args = get_args();

        execute_commands(&args);

    }
}