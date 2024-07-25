pub enum ArgHandle<'a> {
    Exit,
    Print(Vec<&'a str>),
    PrintWorkingDirectory,
    ListContents,
    Concatenate(&'a str),
    Unknown,
}

pub fn handle_args(args : Vec<&str>) -> ArgHandle {
    match Some(args[0]) {
        Some("ex") | Some("e") => ArgHandle::Exit,
        Some("pr") => ArgHandle::Print(args[1..].to_vec()),
        Some("pd") => ArgHandle::PrintWorkingDirectory,
        Some("ls") => ArgHandle::ListContents,
        Some("ct") => ArgHandle::Concatenate(args.get(1).cloned().unwrap_or_default()),
        _ => ArgHandle::Unknown,
    }
}