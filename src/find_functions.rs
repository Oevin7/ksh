use std::{io};
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::Path;

pub fn iterate_dirs(dir : &Path, filters : &Vec<&str>) -> io::Result<()> {

    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                iterate_dirs(&path, &filters)?;
            } else {
                let filename = path.file_name().unwrap().to_string_lossy();

                if !filters.is_empty() && !filters.iter().any(|&filter|
                filename.contains(filter)) {
                    continue;
                }

                if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                    if !filters.is_empty() && !filters.iter()
                        .any(|&filter| ext == &filter[1..]) {
                        continue;
                    }
                }

                println!("{}", filename);

            }

        }
    }

    Ok(())
}