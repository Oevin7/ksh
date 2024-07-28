use crate::config_file_helpers::*;

pub enum ArgHandle<'a> {
    Exit,
    Print(Vec<&'a str>),
    PrintWorkingDirectory,
    ListContents,
    Concatenate(&'a str),
    ChangeDirectory(Vec<&'a str>),
    Find(Vec<&'a str>),
    Filter(Vec<&'a str>),
    Touch(Vec<&'a str>),
    Unknown,
}

pub fn handle_args(args : Vec<&str>) -> ArgHandle {

    let commands = command_args();
    let cmd = args.get(0).map(|s| s.to_lowercase().trim().to_string());

    match cmd {
        Some(c) if c == commands[0] => ArgHandle::Exit,
        Some(c) if c == commands[1] => ArgHandle::Print(args[1..].to_vec()),
        Some(c) if c == commands[2] => ArgHandle::PrintWorkingDirectory,
        Some(c) if c == commands[3] => ArgHandle::ListContents,
        Some(c) if c == commands[4] => ArgHandle::Concatenate(args.get(1).cloned().unwrap_or_default()),
        Some(c) if c == commands[5] => ArgHandle::ChangeDirectory(args[1..].to_vec()),
        Some(c) if c == commands[6] => ArgHandle::Find(args[1..].to_vec()),
        Some(c) if c == commands[7] => ArgHandle::Filter(args[1..].to_vec()),
        Some(c) if c == commands[8] => ArgHandle::Touch(args[1..].to_vec()),
        _ => ArgHandle::Unknown,
    }
}

fn command_args() -> Vec<String> {
    let exit = get_commands("exit").unwrap_or("ex".to_string());
    let print = get_commands("print").unwrap_or("pr".to_string());
    let pd = get_commands("print_working_directory").unwrap_or("pd".to_string());
    let list= get_commands("list_contents").unwrap_or("ls".to_string());
    let ct = get_commands("concatenate").unwrap_or("ct".to_string());
    let cd = get_commands("change_directory").unwrap_or("cd".to_string());
    let find = get_commands("find").unwrap_or("fd".to_string());
    let filter = get_commands("filter").unwrap_or("fl".to_string());
    let touch = get_commands("touch").unwrap_or("tc".to_string());

    let commands : Vec<String> = vec![exit, print, pd, list, ct, cd, find, filter, touch];

    commands

}