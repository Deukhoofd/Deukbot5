use std::collections::HashMap;

use serenity::model::channel::Message;

use command::Command;
use command_group::CommandGroup;
use command_groups::general_commands::*;

use crate::deukbot_result::DeukbotResult;
use crate::message_handling::command_handler::command_data::CommandData;
use crate::message_handling::command_handler::command_request::CommandRequestType;
use crate::message_handling::permission::PermissionLevel;
use serenity::client::Context;
use unicase::UniCase;

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
    static ref COMMAND_LOOKUP: HashMap<UniCase<String>, &'static Command> = {
        let mut h = HashMap::new();
        for cg in &*COMMAND_GROUPS {
            for command in &cg.commands {
                h.insert(UniCase::new(command.get_name().to_string()), command);
                for alternative in command.get_alternatives() {
                    h.insert(UniCase::new(alternative.to_string()), command);
                }
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
        CommandRequestType::OK(c, pars) => {
            let user_permission = match &msg.member {
                None => PermissionLevel::Everyone,
                Some(member) => {
                    super::permission::get_user_permission_level(
                        &ctx,
                        msg.channel_id.to_channel(&ctx).await.unwrap(),
                        &msg.author,
                        &member,
                    )
                    .await
                }
            };
            if user_permission < c.get_permission_level() {
                info!(
                    "Unauthorized user tried to run command: '{}'. User: '{}' with permission: {} < {}",
                    c.get_name(),
                    msg.author.name,
                    user_permission,
                    c.get_permission_level()
                );
                return DeukbotResult::Ok;
            }

            info!(
                "Handling command: '{}' for user: '{}' with permission: {}",
                c.get_name(),
                msg.author.name,
                user_permission
            );
            let res = c
                .run(CommandData {
                    ctx,
                    message: msg.clone(),
                    parameters: pars,
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
