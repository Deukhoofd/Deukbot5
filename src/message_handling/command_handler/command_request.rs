use crate::message_handling::command_handler::command::Command;
use crate::message_handling::command_handler::parameter_matcher::RequestParameter;
use crate::message_handling::permission::PermissionLevel;
use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;
use unicase::UniCase;

lazy_static! {
    static ref COMMAND_NAME_PATTERN: String = {
        format!(
            "(?:<@!?\\d*> !*|^{}+)([^ ]+) *(.*)",
            crate::defines::COMMAND_TRIGGER
        )
    };
    static ref COMMAND_NAME_MATCHER: Regex = Regex::new(&COMMAND_NAME_PATTERN).unwrap();
}

pub enum CommandRequestType {
    OK(&'static Command, Vec<RequestParameter>, PermissionLevel),
    UnknownCommand,
    Invalid,
    Forbidden(&'static Command, PermissionLevel),
    InvalidParameters,
}

impl CommandRequestType {
    pub async fn create(ctx: &Context, msg: &Message) -> CommandRequestType {
        let captures_opt = COMMAND_NAME_MATCHER.captures(&msg.content);
        if captures_opt.is_none() {
            return CommandRequestType::Invalid;
        }
        let captures = captures_opt.unwrap();
        if captures.len() <= 2 {
            return CommandRequestType::Invalid;
        }
        let command_name = captures.get(1).unwrap();
        let command_opt = super::command_groups::COMMAND_LOOKUP
            .get(&UniCase::new(command_name.as_str().to_string()));
        if command_opt.is_none() {
            return CommandRequestType::UnknownCommand;
        }
        let command = command_opt.unwrap();
        if command.forbidden_in_pm() && msg.guild_id.is_none() {
            return CommandRequestType::Forbidden(command, PermissionLevel::Everyone);
        }

        let user_permission = match &msg.member {
            None => PermissionLevel::Everyone,
            Some(member) => {
                super::super::permission::get_user_permission_level(
                    &ctx,
                    msg.channel_id.to_channel(&ctx).await.unwrap(),
                    &msg.author,
                    &member,
                )
                .await
            }
        };
        if user_permission < command.get_permission_level() {
            return CommandRequestType::Forbidden(command, user_permission);
        }

        let parameters =
            CommandRequestType::get_parameters(command, captures.get(2).unwrap().as_str());
        if parameters.is_none() {
            return CommandRequestType::InvalidParameters;
        }

        CommandRequestType::OK(command, parameters.unwrap(), user_permission)
    }

    fn get_parameters(command: &Command, capture: &str) -> Option<Vec<RequestParameter>> {
        for (matcher_index, matcher) in command.get_parameter_matchers().iter().enumerate() {
            let par_captures = matcher.captures(capture);
            if let Some(body) = par_captures {
                let mut a = Vec::new();
                let g = &command.get_parameter_types()[matcher_index];
                for (i, p) in body.iter().skip(1).enumerate() {
                    if i >= g.len() {
                        break;
                    }
                    a.push(RequestParameter {
                        kind: g[i],
                        value: p.unwrap().as_str().to_string(),
                    });
                }
                return Some(a);
            }
        }
        if command.require_parameter_match() {
            None
        } else {
            Some(Vec::new())
        }
    }
}
