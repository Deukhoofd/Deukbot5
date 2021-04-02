use serenity::client::Context;
use serenity::model::channel::Message;

pub struct CommandData {
    pub ctx: Context,
    pub message: Message,
}
