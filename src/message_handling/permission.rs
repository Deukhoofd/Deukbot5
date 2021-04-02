use serenity::client::Context;
use serenity::model::id::{ChannelId, GuildId, UserId};
use serenity::model::prelude::{Channel, User};
use std::collections::HashMap;
use std::fmt::Display;

lazy_static! {
    static ref PERMISSION_CACHE: HashMap<GuildId, HashMap<UserId, PermissionLevel>> =
        HashMap::new();
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Display)]
#[repr(i8)]
pub enum PermissionLevel {
    Banned = -10,
    Bot = -5,
    Everyone = 0,
    Helper = 20,
    Moderator = 40,
    Admin = 60,
    ServerOwner = 80,
    BotCreator = 100,
}

pub async fn get_user_permission_level(
    ctx: &Context,
    channel: Channel,
    user: &User,
) -> PermissionLevel {
    if user.id == crate::global::owner_id() {
        return PermissionLevel::BotCreator;
    }
    if user.bot {
        return PermissionLevel::Bot;
    }
    return match channel {
        Channel::Guild(guild_channel) => {
            if guild_channel
                .guild_id
                .to_partial_guild(ctx)
                .await
                .unwrap()
                .owner_id
                == user.id
            {
                PermissionLevel::ServerOwner
            } else {
                PermissionLevel::Everyone
            }
        }
        _ => PermissionLevel::Everyone,
    };
}
