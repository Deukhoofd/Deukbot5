use crate::message_handling::permission::PermissionLevel;
use serenity::model::id::{GuildId, RoleId};

pub async fn get_role_permission(guild_id: GuildId, role_id: &RoleId) -> Option<PermissionLevel> {
    info!(
        "Fetching permissions for guild {}, role {}",
        guild_id, role_id
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
    for row in rows {
        let v: i16 = row.get(0);
        return Some(num::FromPrimitive::from_i16(v).expect("Permission was invalid number?"));
    }
    return None;
}
