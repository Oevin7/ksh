use std::fs;
use std::env;
use std::ffi::{OsString};
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use crate::arg_handler::{ArgHandle, handle_args};
use crate::file_operations::{find_file, write_file};
use crate::find_functions::{iterate_dirs};
use crate::helper_functions::{split_args};

//Handles the execution of commands in the run_file.rs
pub fn execute_commands(args : &str) {

    let arg = split_args(args);

    if arg.len() == 0 {
        return;
    }

    match handle_args(arg) {
        ArgHandle::Exit => std::process::exit(0),
        ArgHandle::Print(arg) => pr(arg),
        ArgHandle::PrintWorkingDirectory => pd(),
        ArgHandle::ListContents => ls(),
        ArgHandle::Concatenate(arg) => ct(arg),
        ArgHandle::ChangeDirectory(arg) => cd(arg),
        ArgHandle::Find(arg) => fd(arg),
        ArgHandle::Filter(arg) => fl(arg),
        ArgHandle::Touch(arg) => tc(arg),
        ArgHandle::Unknown => eprintln!("Command not found."),
    }

}

//Works the same as echo. Prints whatever argument you give it to the console.
fn pr(args : Vec<&str>) {
    if args.len() == 0 {
        println!("No arguments provided");
    } else {
        for i in 0..args.len() {
            print!("{} ", args[i]);
        }
        println!();
    }
}

//Prints the working directory.
fn pd() {
    let current_dir = env::current_dir().unwrap();
    println!("{}", current_dir.display());
}

//Lists the current files in the directory. Needs formatting done.
fn ls() {
    let current_dir = env::current_dir().unwrap();
    let paths = read_dir(current_dir).unwrap();

    for path in paths {
        print!("{:?} ", path.unwrap().file_name());
    }

    print!("\n");

}

//Works the same as cat on linux
fn ct(args : &str) {
    let arg = split_args(args);
    let mut file_name = String::new();

    if arg.len() == 0 {
        println!("No arguments provided.");
        return
    } else {
        for i in 0..arg.len() {
            file_name = arg[i].parse().unwrap();
        }
    }

    let file_match = find_file(&file_name);

    let file = fs::read_to_string(file_match);

    match file {
        Ok(file) => {
            println!("{}", file);
        }
        Err(e) => eprintln!("An error occurred:  {e}"),
    }

}

//Changes the current directory, works the same as on linux. Needs some formatting done.
fn cd(args : Vec<&str>) {
    let mut file_path = String::new();

    if args.is_empty() {
        file_path = "/home".to_string();
    } else {
        for i in 0..args.len() {
            file_path = args[i].parse().unwrap();
        }
    }

    match env::set_current_dir(&file_path) {
        Ok(_) => {},
        Err(e) => eprintln!("A file or directory was either improperly entered, or does not exist. \
        More information: {}", e),
    }

}

//Similar to find in linux. While not done, it will work in a very similar manner when completed.
fn fd(args : Vec<&str>) {
    let mut current_dir = env::current_dir().unwrap_or_default();
    let mut filters = vec![];
    let mut file_path = String::new();

    if args.len() > 0 {
        let potential_path = Path::new(args[0]);

        if potential_path.exists() {
            file_path = args[0].to_string();

            filters = args[1..].to_vec();
        } else {
            filters = args.to_vec();
        }
    }

    if !file_path.is_empty() {
        current_dir = PathBuf::from(&file_path);
    }

    match iterate_dirs(&current_dir, &filters) {
        Ok(_) => {},
        Err(e) => eprintln!("Could not access the file: {:?}", e),
    }

}

//Filters files in the current directory. Works similar to find, but takes fewer arguments and
//only works in the current directory
fn fl(args : Vec<&str>) {
    let filters: Vec<&str> = args.to_vec();
    let current_dir = env::current_dir().unwrap_or_default();
    let dir = read_dir(&current_dir);

    if filters.is_empty() {
        return;
    }

    match dir {
        Ok(paths) => {
            for files in paths {
                let file = files.unwrap();
                let path = file.path();
                let file_name = file.file_name();
                let ext = path.extension();

                for filter in &filters {
                    if filter.starts_with(".") {
                        if ext.unwrap_or_default() == &OsString::from(&filter[1..]) {
                            println!("{:?}", file_name);
                        }
                    } else {
                        let file_name_str = file_name.to_string_lossy();
                        if file_name_str.contains(filter.trim()) {
                            println!("{:?}", file_name);
                        }
                    }
                }

            }
        }
        Err(e) => eprintln!("Could not access the directory: {e}"),
    }
}

fn tc(args : Vec<&str>) {
    let file_name = args[0];
    let mut file_path = "";

    if args.len() == 2 {
        file_path = args[1];
        match write_file(file_name, file_path) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Could not write to file: {:?}", e);
                return;
            }
        };
    }

    match write_file(file_name, file_path) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Could not write to file: {:?}", e);
            return;
        }
    };

}