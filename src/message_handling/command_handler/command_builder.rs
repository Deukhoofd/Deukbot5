use crate::message_handling::command_handler::async_fn::AsyncFn;
use crate::message_handling::command_handler::command::Command;

pub struct CommandBuilder {
    name: String,
    func: Option<Box<dyn AsyncFn + Send + Sync + 'static>>,
}

impl CommandBuilder {
    pub fn new(name: &str) -> CommandBuilder {
        CommandBuilder {
            name: name.to_string(),
            func: None,
        }
    }

    pub fn with_func(mut self, func: Box<dyn AsyncFn + Send + Sync + 'static>) -> CommandBuilder {
        self.func = Some(func);
        self
    }

    pub fn build(self) -> Command {
        match self.func {
            None => panic!("Command needs to have a function!"),
            _ => {}
        };

        Command::new(&self.name, self.func.unwrap())
    }
}
