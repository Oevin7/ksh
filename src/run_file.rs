use std::io;
use std::io::Write;
use crate::shell_functions::execute_commands;
use crate::helper_functions::get_args;
use crate::config_file_helpers::*;

pub fn run() {

    let file = get_config_path();

    loop {

        if !file.exists() {
            let config = init_config();
            /*match write_commands(config) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Could not write to file: {:?}", e);
                }
            };*/
        }

        let user = get_user().unwrap_or_else(|| "user".to_string());

        print!("ksh@{user}$> ", );
        io::stdout().flush().unwrap();

        let args = get_args();

        execute_commands(&args);

    }
}

