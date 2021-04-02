use crate::message_handling::command_handler::async_fn::AsyncFn;
use crate::message_handling::command_handler::command::Command;
use crate::message_handling::command_handler::parameter_matcher::ParameterType;
use crate::message_handling::permission::PermissionLevel;

pub struct CommandBuilder {
    name: String,
    alternatives: Vec<String>,
    permission_level: PermissionLevel,
    short_help: Option<String>,
    long_help: Option<String>,
    func: Option<Box<dyn AsyncFn + Send + Sync + 'static>>,
    pars: Vec<Vec<ParameterType>>,
}

impl CommandBuilder {
    pub fn new(name: &str, permission_level: PermissionLevel) -> CommandBuilder {
        CommandBuilder {
            name: name.to_string(),
            alternatives: Vec::new(),
            permission_level,
            short_help: None,
            long_help: None,
            func: None,
            pars: Vec::new(),
        }
    }

    pub fn with_alternative(mut self, name: &str) -> CommandBuilder {
        self.alternatives.push(name.to_string());
        self
    }

    pub fn with_help(mut self, short_help: &str, long_help: &str) -> CommandBuilder {
        self.short_help = Some(short_help.to_string());
        self.long_help = Some(long_help.to_string());
        self
    }

    pub fn with_func(mut self, func: Box<dyn AsyncFn + Send + Sync + 'static>) -> CommandBuilder {
        self.func = Some(func);
        self
    }

    pub fn with_parameters(mut self, pars: Vec<ParameterType>) -> CommandBuilder {
        self.pars.push(pars);
        self
    }

    pub fn build(self) -> Command {
        if self.func.is_none() {
            panic!("Command needs to have a function!")
        };

        Command::new(
            &self.name,
            self.alternatives,
            self.permission_level,
            self.func.unwrap(),
            self.pars,
            self.short_help,
            self.long_help,
        )
    }
}
