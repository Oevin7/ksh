use std::{env, fs, io};
use std::fs::read_dir;
use std::path::Path;

pub fn basic_fn() {
    let current_dir = env::current_dir().unwrap_or_default();

    let paths = read_dir(current_dir).unwrap();

    //need to introduce proper error handling in the future.
    for path in paths {
        println!("{:?}", path.unwrap().file_name());
    }
}

pub fn iterate_dirs(dir : &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                iterate_dirs(&path).unwrap_or_default();
            } else {
                println!("{:?}", path.file_name());
            }

        }
    }

    Ok(())
}