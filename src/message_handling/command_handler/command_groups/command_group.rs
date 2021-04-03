use crate::message_handling::command_handler::command::Command;

pub struct CommandGroup {
    pub name: String,
    pub commands: Vec<Command>,
}
