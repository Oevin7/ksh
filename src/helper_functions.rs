pub fn combine_str(args : Vec<&str>) -> String {

    let mut return_str = String::new();

    for i in 0..args.len() {
        return_str = args[i].parse().unwrap();
    }

    return_str

}

//Gets arguments for the shell
pub fn get_args() -> String {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => eprintln!("error: {}", error),
    }
    input
}

//Splits the white space of arguments for easy parsing and editing.
pub(crate) fn split_args(args : &str) -> Vec<&str> {
    args.split_whitespace().collect()
}