use crate::message_handling::command_handler::parameter_matcher::RequestParameter;
use crate::message_handling::permission::PermissionLevel;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::prelude::CurrentUser;

pub struct CommandData {
    pub ctx: Context,
    pub message: Message,
    pub parameters: Vec<RequestParameter>,
    pub permission: PermissionLevel,
    pub current_user: CurrentUser,
}
