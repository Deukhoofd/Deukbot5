use regex::Regex;
use serenity::client::Context;
use serenity::futures::StreamExt;
use serenity::http::CacheHttp;
use serenity::model::guild::Member;
use serenity::model::id::{GuildId, UserId};
use serenity::model::user::User;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub enum ParameterType {
    Word,
    Number,
    Remainder,
    User,
    Timespan,
}

pub struct RequestParameter {
    pub(crate) kind: ParameterType,
    pub(crate) value: String,
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
    pub async fn as_discord_guild_user(&self, ctx: &Context, guild: &GuildId) -> Option<Member> {
        let mut cached_guild = guild.to_guild_cached(ctx).await;

        if let Some(v) = self.as_u64() {
            let t1 = chrono::Utc::now();
            if let Some(guild) = &mut cached_guild {
                if let Some(user) = guild.members.get(&UserId(v)) {
                    trace!(
                        "Time fetching member from cache: {}",
                        ((chrono::Utc::now() - t1).num_milliseconds())
                    );
                    return Some(user.clone());
                }
            }
            if let Ok(user) = ctx.http().get_member(guild.0, v).await {
                trace!(
                    "Time fetching member over http: {}",
                    ((chrono::Utc::now() - t1).num_milliseconds())
                );
                if cached_guild.is_some() {
                    cached_guild
                        .unwrap()
                        .members
                        .insert(UserId(v), user.clone());
                }
                return Some(user);
            }
        };
        if let Some(guild) = cached_guild {
            match guild.member_named(self.value.as_str()) {
                None => {}
                Some(m) => {
                    return Some(m.clone());
                }
            }
        }
        let mut members = guild.members_iter(ctx.http()).boxed();
        while let Some(member_result) = members.next().await {
            if let Ok(member) = member_result {
                if member.user.name.eq_ignore_ascii_case(self.value.as_str()) {
                    return Some(member);
                }
            }
        }
        None
    }

    pub async fn as_discord_user(&self, ctx: &Context) -> Option<User> {
        match self.as_u64() {
            Some(v) => match UserId(v).to_user(ctx).await {
                Ok(u) => Some(u),
                _ => None,
            },
            None => None,
        }
    }
}

fn get_parameter_regex(t: &ParameterType) -> &str {
    match t {
        ParameterType::Word => (" *(\\w+)"),
        ParameterType::Number => (" *(\\d+)(?:$| |\n)"),
        ParameterType::Remainder => (" *(.*)"),
        ParameterType::User => (" *(?:<@!*(\\d+)>|(\\d+)|(\\w+)(?:$| |\n))"),
        ParameterType::Timespan => (" *(?\\d+\\.*\\d*[smhd])"),
    }
}

pub fn generate_parameter_regex(pars: &[Vec<ParameterType>]) -> Vec<Regex> {
    let mut a = Vec::new();
    for par_types in pars {
        let mut s = String::new();
        for t in par_types {
            s = s.add(get_parameter_regex(t));
        }
        a.push(Regex::new(s.as_str()).unwrap());
    }

    a
}
