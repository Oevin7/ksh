use std::fs;
use std::env;
use std::fs::read_dir;
use crate::arg_handler::{ArgHandle, handle_args};
use crate::file_operations::find_file;
use crate::helper_functions::{get_args, combine_str, split_args};

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
    let paths = fs::read_dir(current_dir).unwrap();

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

    for i in 0..args.len() {
        file_path = args[i].parse().unwrap();
    }

    match env::set_current_dir(file_path) {
        Ok(_) => {},
        Err(e) => eprintln!("A file or directory was either improperly entered, or does not exist. \
        More information: {e}"),
    }

}

//Similar to find in linux. While not done, it will work in a very similar manner when completed.
fn fd(args : &str) {
    let arg = split_args(args);

    let current_dir = env::current_dir().unwrap_or_default();

    if arg.len() == 0 {
        let paths = read_dir(current_dir).unwrap();

        //need to introduce proper error handling in the future.
        for path in paths {
            println!("{:?}", path.unwrap().file_name());
        }
    } else if arg.len() == 1 {
        let file_path = combine_str(arg);
        let paths = read_dir(file_path).unwrap();

        //need to introduce proper error handling in the future.
        for path in paths {
            println!("{:?}", path.unwrap().file_name());
        }
    }
}