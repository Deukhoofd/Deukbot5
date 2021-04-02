use std::collections::HashMap;

use serenity::model::channel::Message;

use command::Command;
use command_group::CommandGroup;
use command_groups::general_commands::*;

use crate::deukbot_result::DeukbotResult;
use crate::message_handling::command_handler::command_data::CommandData;
use crate::message_handling::command_handler::command_request::CommandRequestType;
use serenity::client::Context;
use serenity::Error;

pub(crate) mod async_fn;
pub mod command;
pub mod command_builder;
pub mod command_data;
mod command_group;
pub mod command_groups;
pub mod command_request;
mod parameter_matcher;

const COMMAND_TRIGGER: char = '~';

lazy_static! {
    static ref COMMAND_GROUPS: Vec<&'static CommandGroup> = vec![&*GENERAL_COMMANDS];
    static ref COMMAND_LOOKUP: HashMap<String, &'static Command> = {
        let mut h = HashMap::new();
        for cg in &*COMMAND_GROUPS {
            for command in &cg.commands {
                h.insert(command.get_name().to_string(), command);
            }
        }

        h
    };
}

pub fn setup_commands() {
    lazy_static::initialize(&COMMAND_GROUPS);
    lazy_static::initialize(&COMMAND_LOOKUP);
}

pub async fn handle_message(ctx: Context, msg: &Message) -> DeukbotResult {
    if msg.content.is_empty() {
        return DeukbotResult::Ok;
    }
    // Message needs to start with the command trigger, or mention the bot user.
    if msg.content.chars().next().unwrap() != COMMAND_TRIGGER
        && !msg.mentions_user_id(crate::global::deukbot_id())
    {
        return DeukbotResult::Ok;
    }

    let cmd = CommandRequestType::create(msg);
    match cmd {
        CommandRequestType::OK(c) => {
            // TODO: Check whether allowed in this channel.

            let res = c
                .run(CommandData {
                    ctx,
                    message: msg.clone(),
                })
                .await;
            return match res {
                Ok(_) => DeukbotResult::Ok,
                Err(e) => DeukbotResult::Err(e.to_string()),
            };
        }
        CommandRequestType::UnknownCommand => {
            // TODO: This needs similar command check
            error!("Unknown command")
        }
        CommandRequestType::Invalid => {
            error!("Invalid content: {}", msg.content);
        }
        CommandRequestType::Forbidden => {
            info!(
                "Unauthorized user tried to run command: {} -> {}",
                msg.author.name, msg.content
            );
        }
        CommandRequestType::InvalidParameters => {
            error!("Invalid parameters: {}", msg.content);
        }
    }

    DeukbotResult::Ok
}
