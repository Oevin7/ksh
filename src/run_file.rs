use std::io;
use std::io::Write;
use crate::shell_functions::execute_commands;
use crate::helper_functions::get_args;

pub fn run() {
    loop {
        print!("KevShell$> ");
        io::stdout().flush().unwrap();

        let args = get_args();

        execute_commands(&args);

    }
}