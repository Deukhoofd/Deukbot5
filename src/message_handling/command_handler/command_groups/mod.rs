use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::http::Http;
use serenity::model::prelude::{ChannelId, Message};

pub mod command_group;
pub mod general_commands;

use super::command::Command;
use crate::message_handling::permission::PermissionLevel;
use command_group::CommandGroup;
use std::collections::HashMap;
use std::ops::Add;
use unicase::UniCase;

lazy_static! {
    static ref COMMAND_GROUPS: Vec<&'static CommandGroup> =
        vec![&*general_commands::GENERAL_COMMANDS];
    pub static ref COMMAND_LOOKUP: HashMap<UniCase<String>, &'static Command> = {
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

pub async fn send_message<'a, F>(
    channel_id: &ChannelId,
    http: impl AsRef<Http>,
    f: F,
) -> Result<Message, serenity::Error>
where
    for<'b> F: FnOnce(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a>,
{
    let start_time = chrono::Utc::now();
    let res = channel_id.send_message(http, f).await;
    trace!(
        "Sending message took: {} ms",
        ((chrono::Utc::now() - start_time).num_milliseconds())
    );
    res
}

pub fn build_help_function(embed: &mut CreateEmbed, permission: PermissionLevel) {
    for command_group in COMMAND_GROUPS.iter() {
        let mut s = String::new();
        for command in command_group.commands.iter() {
            if command.get_short_help().is_none() {
                continue;
            }
            if command.get_permission_level() < permission {
                s = s.add(
                    format!(
                        "**{}**: {}\n",
                        command.get_name(),
                        command.get_short_help().as_ref().unwrap()
                    )
                    .as_str(),
                );
            }
        }
        if !s.is_empty() {
            embed.field(command_group.name.to_string(), s, false);
        }
    }
}
