use std::{io};
use std::path::PathBuf;
use ini::Ini;

pub fn init_config() -> Ini {
    let mut config = Ini::new();
    config.with_section(Some("User"))
        .set("name", "user");
    config.with_section(Some("Commands"))
        .set("exit", "ex")
        .set("print", "pr")
        .set("print_working_directory", "pd")
        .set("list_contents", "ls")
        .set("concatenate", "ct")
        .set("change_directory", "cd")
        .set("find", "fd")
        .set("filter", "fl")
        .set("touch", "tc");

    config

}

pub fn write_commands(config : Ini) -> Result<(), io::Error> {
    config.write_to_file(get_config_path())?;
    Ok(())
}

pub fn get_commands(command : &str) -> Option<String> {
    let mut config = match Ini::load_from_file(get_config_path())  {
        Ok(con) => con,
        Err(e) => {
            eprintln!("Could not access file: {:?}", e);
            return None
        }
    };

    let section = match config.section_mut(Some("Commands")) {
        Some(val) => val,
        None => {
            eprintln!("Could not access the values");
            return None
        }
    };

    for (key, val) in section.iter() {
        if key == command {
            return Some(val.to_string());
        }
    }

    None

}

pub fn get_user() -> Option<String> {
    let mut config = match Ini::load_from_file(get_config_path()) {
        Ok(con) => con,
        Err(e) => {
            eprintln!("Could not access file: {e}");
            return None
        }
    };

    let section = match config.section_mut(Some("User")) {
        Some(val) => val,
        None => {
            eprintln!("Could not access the values");
            return None
        }
    };

    let user = section.get("name")?.to_string();

    Some(user)

}

pub fn get_config_path() -> PathBuf {
    let path = PathBuf::from("/home/kevin/Documents/GitHub/ksh/ksh.ini");
    path
}