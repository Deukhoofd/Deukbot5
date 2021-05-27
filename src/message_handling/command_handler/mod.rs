use serenity::client::Context;
use serenity::model::channel::Message;

use crate::deukbot_result::DeukbotResult;
use crate::message_handling::command_handler::command_data::CommandData;
use crate::message_handling::command_handler::command_request::CommandRequestType;

pub(crate) mod async_fn;
pub mod command;
pub mod command_builder;
pub mod command_data;
pub mod command_groups;
pub mod command_request;
mod parameter_matcher;
pub mod slash_command_handling;

pub async fn handle_message(ctx: Context, msg: &Message) -> DeukbotResult {
    if msg.content.is_empty() {
        return DeukbotResult::Ok;
    }
    // Message needs to start with the command trigger, or mention the bot user.
    if msg.content.chars().next().unwrap() != crate::defines::COMMAND_TRIGGER
        && !msg.mentions_user_id(ctx.cache.current_user_id().await)
    {
        return DeukbotResult::Ok;
    }

    let start_time = chrono::Utc::now();
    let cmd = CommandRequestType::create(&ctx, msg).await;
    trace!(
        "Command request creation time: {} ms",
        ((chrono::Utc::now() - start_time).num_milliseconds())
    );
    match cmd {
        CommandRequestType::OK(c, pars, user_permission) => {
            info!(
                "Handling command: '{}' for user: '{}' with permission: {}",
                c.get_name(),
                msg.author.name,
                user_permission
            );

            let cmd_start_time = chrono::Utc::now();
            let current_user = ctx.cache.current_user().await;
            let res = c
                .run(CommandData {
                    ctx,
                    message: msg.clone(),
                    parameters: pars,
                    permission: user_permission,
                    current_user,
                })
                .await;
            trace!(
                "Command run time for command: '{}': {} ms",
                c.get_name(),
                ((chrono::Utc::now() - cmd_start_time).num_milliseconds())
            );
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
        CommandRequestType::Forbidden(command, user_permission) => {
            info!(
                "Unauthorized user tried to run command: '{}'. User: '{}' with permission: {} < {}",
                command.get_name(),
                msg.author.name,
                user_permission,
                command.get_permission_level(),
            );
        }
        CommandRequestType::InvalidParameters => {
            error!("Invalid parameters: {}", msg.content);
        }
    }

    DeukbotResult::Ok
}
