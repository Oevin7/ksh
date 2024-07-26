use std::{env, fs, io};
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::Path;

pub fn iterate_dirs(dir : &Path, filter : Vec<&str>) -> io::Result<()> {

    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                iterate_dirs(&path, filter.to_vec()).unwrap_or_default();
            } else {
                if filter.is_empty() {
                    match path.file_name() {
                        Some(name) => println!("{:?}", name),
                        None => eprintln!("Invalid filename at {:?}", path),
                    }
                }
            }

        }
    }

    Ok(())
}