use std::{io};
use std::ffi::OsString;
use std::fs::read_dir;
use std::path::Path;

pub fn iterate_dirs(dir : &Path, filters : Vec<&str>) -> io::Result<()> {

    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = entry.file_name();
            let ext = path.extension();

            if path.is_dir() {
                iterate_dirs(&path, filters.to_vec()).unwrap_or_default();
            } else {
                if filters.is_empty() {
                    match path.file_name() {
                        Some(name) => println!("{:?}", name),
                        None => eprintln!("Invalid filename at {:?}", path),
                    }
                } else {
                    for filter in &filters {
                        if filter.starts_with(".") {
                            if ext.unwrap_or_default() == &OsString::from(&filter[1..]) {
                                println!("{:?}", file_name);
                            }
                        }
                    }
                }
            }

        }
    }

    Ok(())
}