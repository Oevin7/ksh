use std::fs;
use std::env;
use std::path::PathBuf;
use crate::arg_handler::{ArgHandle, handle_args};

pub fn get_args() -> String {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => eprintln!("error: {}", error),
    }
    input
}

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
        ArgHandle::Unknown => eprintln!("Command not found."),
    }

}

fn split_args(args : &str) -> Vec<&str> {
    args.split_whitespace().collect()
}

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

fn pd() {
    let current_dir = env::current_dir().unwrap();
    println!("{}", current_dir.display());
}

fn ls() {
    let current_dir = env::current_dir().unwrap();
    let paths = fs::read_dir(current_dir).unwrap();

    for path in paths {
        print!("{:?} ", path.unwrap().file_name());
    }

    print!("\n");

}

fn find_file(name : &str) -> PathBuf {
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