use std::{env, fs};
use std::path::PathBuf;

pub fn find_file(name : &str) -> PathBuf {
    let current_dir = env::current_dir().unwrap();
    let paths = fs::read_dir(current_dir).unwrap();

    let mut return_file = PathBuf::new();

    for path in paths {
        match path {
            Ok(path) => {
                if path.file_name() == name {
                    return_file = path.path();
                    break
                }
            }
            Err(e) => eprintln!("An error occurred: {e}"),
        }
    }

    return_file

}