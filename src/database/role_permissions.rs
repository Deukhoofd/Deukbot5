use crate::message_handling::permission::PermissionLevel;
use serenity::client::Context;
use serenity::model::id::{GuildId, RoleId};

pub async fn get_role_permission(
    ctx: &Context,
    guild_id: GuildId,
    role_id: &RoleId,
) -> Option<PermissionLevel> {
    let start_time = chrono::Utc::now();
    info!(
        "Fetching permissions for guild {}, role {}",
        guild_id.name(ctx).await.unwrap(),
        match role_id.to_role_cached(ctx).await {
            None => role_id.to_string(),
            Some(v) => v.name,
        }
    );

    let conn = super::get_connection().await;
    let rows = conn
        .client
        .query(
            "SELECT permission_level FROM permission_roles \
                            WHERE server_id = $1::bigint \
                              AND role_id   = $2::bigint",
            &[
                &(i64::from_ne_bytes(guild_id.0.to_ne_bytes())),
                &(i64::from_ne_bytes(role_id.0.to_ne_bytes())),
            ],
        )
        .await
        .expect("Failed to retrieve permission levels from database");
    if !rows.is_empty() {
        let v: i16 = rows[0].get(0);
        return Some(num::FromPrimitive::from_i16(v).expect("Permission was invalid number?"));
    }
    trace!(
        "Getting permission level db time: {} ms",
        (chrono::Utc::now() - start_time).num_milliseconds()
    );
    return None;
}
