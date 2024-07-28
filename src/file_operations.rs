use std::{env, fs, io};
use std::path::{Path, PathBuf};

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

pub fn write_file(file_name : &str, file_path : &str) -> Result<(), io::Error> {
    if file_path.is_empty() {
        match fs::File::create(&file_name) {
            Ok(_) => {},
            Err(e) => eprintln!("Could not create file: {:?}", e),
        };
    } else {
        let full_path = Path::new(file_path).join(file_name);
        match fs::File::create(&full_path) {
            Ok(_) => {},
            Err(e) => eprintln!("Could not create file: {:?}", e),
        }
    }

    Ok(())

}

