use serenity::client::Context;
use serenity::model::channel::GuildChannel;
use serenity::model::guild::PartialMember;
use serenity::model::id::{GuildId, RoleId, UserId};
use serenity::model::prelude::{Channel, User};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Mutex;

lazy_static! {
    static ref PERMISSION_CACHE: Mutex<HashMap<GuildId, HashMap<RoleId, PermissionLevel>>> =
        Mutex::new(HashMap::new());
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Display, FromPrimitive)]
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

async fn get_db_user_permission_level_for_role(
    ctx: &Context,
    guild: GuildId,
    role: &RoleId,
) -> PermissionLevel {
    {
        let mut guard = PERMISSION_CACHE.lock().unwrap();
        match guard.get(&guild) {
            Some(g) => {
                if let Some(p) = g.get(&role) {
                    return *p;
                };
            }
            None => {
                guard.insert(guild, HashMap::new());
            }
        };
    }
    let permission = crate::database::role_permissions::get_role_permission(ctx, guild, role).await;
    let final_permission = match permission {
        None => PermissionLevel::Everyone,
        Some(v) => v,
    };
    {
        let mut guard = PERMISSION_CACHE.lock().unwrap();
        guard
            .get_mut(&guild)
            .unwrap()
            .insert(*role, final_permission);
    }
    final_permission
}

async fn get_guild_owner(guild_channel: &GuildChannel, ctx: &Context) -> UserId {
    match guild_channel.guild(ctx).await {
        None => {
            guild_channel
                .guild_id
                .to_partial_guild(ctx)
                .await
                .unwrap()
                .owner_id
        }
        Some(g) => g.owner_id,
    }
}

pub async fn get_user_permission_level(
    ctx: &Context,
    channel: Channel,
    user: &User,
    member: &PartialMember,
) -> PermissionLevel {
    if user.id == crate::global::owner_id() {
        return PermissionLevel::BotCreator;
    }
    if user.bot {
        return PermissionLevel::Bot;
    }
    return match channel {
        Channel::Guild(guild_channel) => {
            if get_guild_owner(&guild_channel, ctx).await == user.id {
                PermissionLevel::ServerOwner
            } else {
                let mut highest_permission = PermissionLevel::Banned;
                for role in &member.roles {
                    let perm =
                        get_db_user_permission_level_for_role(&ctx, guild_channel.guild_id, role)
                            .await;
                    if perm > highest_permission {
                        highest_permission = perm;
                    }
                }
                highest_permission
            }
        }
        _ => PermissionLevel::Everyone,
    };
}
