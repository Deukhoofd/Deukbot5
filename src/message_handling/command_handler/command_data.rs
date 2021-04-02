use crate::message_handling::command_handler::parameter_matcher::RequestParameter;
use serenity::client::Context;
use serenity::model::channel::Message;

pub struct CommandData {
    pub ctx: Context,
    pub message: Message,
    pub parameters: Vec<RequestParameter>,
}
