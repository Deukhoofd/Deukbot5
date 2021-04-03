use serenity::builder::CreateMessage;
use serenity::http::Http;
use serenity::model::prelude::{ChannelId, Message};

pub mod command_group;
pub mod general_commands;

use super::command::Command;
use command_group::CommandGroup;
use std::collections::HashMap;
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
