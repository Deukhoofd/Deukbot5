use regex::Regex;
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
    pub async fn as_discord_guild_user(
        &self,
        ctx: impl CacheHttp,
        guild: &GuildId,
    ) -> Option<Member> {
        match self.as_u64() {
            Some(v) => match guild.member(&ctx, UserId(v)).await {
                Ok(user) => {
                    return Some(user);
                }
                _ => {}
            },
            None => {}
        };
        let mut members = guild.members_iter(ctx.http()).boxed();
        while let Some(member_result) = members.next().await {
            match member_result {
                Ok(member) => {
                    if member.user.name.eq_ignore_ascii_case(self.value.as_str()) {
                        return Some(member);
                    }
                }
                Err(_) => {}
            }
        }
        None
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

fn get_parameter_regex(t: &ParameterType, index: usize) -> String {
    match t {
        ParameterType::Word => format!(" *(?<par{}>\\w+)", index),
        ParameterType::Number => format!(" *(?<par{}>\\d+)(?:$| |\n)", index),
        ParameterType::Remainder => format!(" *(?<par{}>.*)", index),
        ParameterType::User => format!(" *(?:<@!*(\\d+)>|(\\d+)|(\\w+)(?:$| |\n))"),
        ParameterType::Timespan => format!(" *(?<par{}>\\d+\\.*\\d*[smhd])", index),
    }
}

pub fn generate_parameter_regex(pars: &Vec<Vec<ParameterType>>) -> Vec<Regex> {
    let mut a = Vec::new();
    for par_types in pars {
        let mut s = String::new();
        for (i, t) in par_types.iter().enumerate() {
            s = s.add(get_parameter_regex(t, i).as_str());
        }
        a.push(Regex::new(s.as_str()).unwrap());
    }

    a
}
