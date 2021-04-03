use crate::message_handling::command_handler::async_fn::AsyncFn;
use crate::message_handling::command_handler::command_data::CommandData;
use crate::message_handling::command_handler::parameter_matcher::{
    generate_parameter_regex, ParameterType,
};
use crate::message_handling::permission::PermissionLevel;
use regex::Regex;
use serenity::Error;

pub struct Command {
    name: String,
    alternatives: Vec<String>,
    permission_level: PermissionLevel,
    short_help: Option<String>,
    long_help: Option<String>,
    parameter_types: Vec<Vec<ParameterType>>,
    forbid_in_pm: bool,
    require_parameter_match: bool,
    parameter_matchers: Vec<Regex>,
    func: Box<dyn AsyncFn + Send + Sync + 'static>,
}

impl Command {
    pub fn new(
        name: &str,
        alternatives: Vec<String>,
        permission_level: PermissionLevel,
        func: Box<dyn AsyncFn + Send + Sync + 'static>,
        parameters: Vec<Vec<ParameterType>>,
        short_help: Option<String>,
        long_help: Option<String>,
        require_parameter_match: bool,
    ) -> Command {
        let matchers = generate_parameter_regex(&parameters);
        Command {
            name: name.to_string(),
            alternatives,
            permission_level,
            short_help,
            long_help,
            parameter_types: parameters,
            forbid_in_pm: false,
            require_parameter_match,
            parameter_matchers: matchers,
            func,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_alternatives(&self) -> &Vec<String> {
        &self.alternatives
    }
    pub fn get_permission_level(&self) -> PermissionLevel {
        self.permission_level
    }
    pub fn get_parameter_types(&self) -> &Vec<Vec<ParameterType>> {
        &self.parameter_types
    }
    pub fn get_parameter_matchers(&self) -> &Vec<Regex> {
        &self.parameter_matchers
    }

    pub fn get_short_help(&self) -> &Option<String> {
        &self.short_help
    }
    pub fn get_long_help(&self) -> &Option<String> {
        &self.long_help
    }
    pub fn require_parameter_match(&self) -> bool {
        self.require_parameter_match
    }

    pub async fn run<'a>(&self, req: CommandData) -> Result<(), Error> {
        self.func.call(req).await
    }
}
