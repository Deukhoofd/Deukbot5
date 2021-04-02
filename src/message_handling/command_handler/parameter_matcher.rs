use serenity::http::CacheHttp;
use serenity::model::guild::{Guild, Member};
use serenity::model::id::UserId;
use serenity::model::user::User;

pub enum ParameterType {
    Word,
    Number,
    Remainder,
    User,
    Timespan,
}

pub struct RequestParameter {
    kind: ParameterType,
    value: String,
}

impl RequestParameter {
    pub fn as_string(&self) -> String {
        self.value.clone()
    }
    pub fn as_int(&self) -> Option<i32> {
        match self.value.parse() {
            Ok(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_u64(&self) -> Option<u64> {
        match self.value.parse() {
            Ok(v) => Some(v),
            _ => None,
        }
    }
    pub async fn as_discord_guild_user(
        &self,
        ctx: impl CacheHttp,
        guild: &Guild,
    ) -> Option<Member> {
        match self.as_u64() {
            Some(v) => match guild.member(ctx, UserId(v)).await {
                Ok(user) => {
                    return Some(user);
                }
                _ => {}
            },
            None => {}
        };
        match guild.member_named(self.value.as_str()) {
            Some(m) => Some(m.clone()),
            None => None,
        }
    }

    pub async fn as_discord_user(&self, ctx: impl CacheHttp) -> Option<User> {
        match self.as_u64() {
            Some(v) => match UserId(v).to_user(ctx).await {
                Ok(u) => Some(u),
                _ => None,
            },
            None => None,
        }
    }
}
