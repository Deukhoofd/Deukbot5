use crate::message_handling::command_handler::command::Command;
use regex::Regex;
use serenity::model::channel::Message;

lazy_static! {
    static ref COMMAND_NAME_PATTERN: String = {
        format!(
            "(?:<@!?\\d*> !*|^{}+)([^ ]+) *(.*)",
            crate::message_handling::command_handler::COMMAND_TRIGGER
        )
    };
    static ref COMMAND_NAME_MATCHER: Regex = Regex::new(&COMMAND_NAME_PATTERN).unwrap();
}

pub enum CommandRequestType {
    OK(&'static Command),
    UnknownCommand,
    Invalid,
    Forbidden,
    InvalidParameters,
}

impl CommandRequestType {
    pub fn create(msg: &Message) -> CommandRequestType {
        let captures_opt = COMMAND_NAME_MATCHER.captures(&msg.content);
        if captures_opt.is_none() {
            return CommandRequestType::Invalid;
        }
        let captures = captures_opt.unwrap();
        if captures.len() <= 2 {
            return CommandRequestType::Invalid;
        }
        let command_name = captures.get(1).unwrap();
        let command_opt = super::COMMAND_LOOKUP.get(command_name.as_str());
        if command_opt.is_none() {
            return CommandRequestType::UnknownCommand;
        }

        // TODO: Permissions

        // TODO: Parameters

        CommandRequestType::OK(command_opt.unwrap())
    }
}
