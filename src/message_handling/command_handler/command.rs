use crate::message_handling::command_handler::async_fn::AsyncFn;
use crate::message_handling::command_handler::command_data::CommandData;
use crate::message_handling::command_handler::parameter_matcher::ParameterType;
use serenity::Error;

pub struct Command {
    name: String,
    alternatives: Vec<String>,
    // TODO: PermissionLevel
    short_help: Option<String>,
    long_help: Option<String>,
    has_help: bool,
    parameter_types: Vec<Vec<ParameterType>>,
    forbid_in_pm: bool,
    require_parameter_match: bool,

    parameter_matchers: Vec<String>,

    //func: fn(&CommandRequest) -> Pin<Box<dyn Future<Output = ()>>>,
    func: Box<dyn AsyncFn + Send + Sync + 'static>,
    // func: Box<dyn Fn(&CommandRequest) + Send + Sync + 'static>,
}

impl Command {
    pub fn new(name: &String, func: Box<dyn AsyncFn + Send + Sync + 'static>) -> Command {
        Command {
            name: name.to_string(),
            alternatives: vec![],
            short_help: None,
            long_help: None,
            has_help: false,
            parameter_types: vec![],
            forbid_in_pm: false,
            require_parameter_match: false,
            parameter_matchers: vec![],
            func,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub async fn run<'a>(&self, req: CommandData) -> Result<(), Error> {
        self.func.call(req).await
    }
}
