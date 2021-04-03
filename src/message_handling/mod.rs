use crate::deukbot_result::DeukbotResult;
use serenity::client::Context;
use serenity::model::prelude::Message;

pub mod command_handler;
pub mod permission;

pub async fn handle_message(ctx: Context, msg: Message) -> DeukbotResult {
    if msg.author.bot {
        return DeukbotResult::Ok;
    }
    if msg.author.id == ctx.cache.current_user_id().await {
        return DeukbotResult::Ok;
    }

    trace!("{}", msg.content);
    command_handler::handle_message(ctx, &msg).await;

    DeukbotResult::Ok
}
