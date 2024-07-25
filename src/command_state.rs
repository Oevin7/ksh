pub enum CommandState {
    Single,
    Quoted,
    Empty,
}

impl CommandState {
    pub fn new() -> CommandState {
        CommandState::Empty
    }

}