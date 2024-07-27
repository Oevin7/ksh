use std::io;
use std::io::Write;
use std::path::Path;
use crate::shell_functions::execute_commands;
use crate::helper_functions::get_args;
use crate::config_file_helpers::*;

pub fn run() {
    loop {

        let file = Path::new("ksh.ini");

        if !file.exists() {
            let config = init_config();
            write_commands(config).unwrap();
        }

        let user = get_user().unwrap_or_else(|| "user".to_string());

        print!("ksh@{user}$> ", );
        io::stdout().flush().unwrap();

        let args = get_args();

        execute_commands(&args);

    }
}

