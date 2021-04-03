use super::super::command_builder::CommandBuilder;
use super::super::command_group::CommandGroup;
use crate::embed::{set_default_style, setup_embed};
use crate::message_handling::command_handler::command_data::CommandData;
use crate::message_handling::command_handler::parameter_matcher::ParameterType;
use crate::message_handling::permission::PermissionLevel;
use serenity::model::user::User;
use serenity::Error;

lazy_static! {
    pub static ref GENERAL_COMMANDS: CommandGroup = {
        CommandGroup {
            commands: vec![
                CommandBuilder::new("info", PermissionLevel::Everyone)
                    .with_help("Gives basic info on the bot", "Gives basic info on the bot")
                    .with_func(Box::new(info))
                    .build(),
                CommandBuilder::new("ping", PermissionLevel::Everyone)
                    .with_help("Ping Pong Response", "Generates a simple Pong response when triggered, and report on the response times")
                    .with_func(Box::new(ping))
                    .build(),
                CommandBuilder::new("avatar", PermissionLevel::Everyone)
                    .with_alternative("pfp")
                    .with_help("Gets a users avatar", "Gets a users avatar. Returns avatar of user using the command if no user was specified.")
                    .with_func(Box::new(avatar))
                    .with_parameters(vec![ParameterType::User])
                    .build(),
            ],
        }
    };
}

async fn info(req: CommandData) -> Result<(), Error> {
    req.message
        .channel_id
        .send_message(&req.ctx, |m| {
            m.embed(|e| {
                setup_embed(e, "Deukbot", "A bot designed by Deukhoofd")
                    .description("A bot designed by Deukhoofd")
                    .field("Software", "Deukbot 5.0", true)
                    .field("Creator", "<@84372569012043776>", true)
            })
        })
        .await?;
    Ok(())
}

async fn ping(req: CommandData) -> Result<(), Error> {
    let t = chrono::Utc::now();
    let msg_promise = req.message.channel_id.send_message(&req.ctx, |m| {
        m.embed(|e| {
            setup_embed(e, "Pong", "Pong").field(
                "Measured Ping between Message and Command Handling",
                format!("{} ms", (t - req.message.timestamp).num_milliseconds()),
                false,
            )
        })
    });
    let t2 = chrono::Utc::now();
    let mut msg = msg_promise.await?;
    let ts = msg.timestamp;
    msg.edit(&req.ctx, |m| {
        m.embed(|e| {
            setup_embed(e, "Pong", "Pong")
                .field(
                    "Measured Ping between Message and Command Handling",
                    format!("{} ms", (t - req.message.timestamp).num_milliseconds()),
                    false,
                )
                .field(
                    "Measured Time handling embed creation",
                    format!("{} ms", (t2 - t).num_milliseconds()),
                    false,
                )
                .field(
                    "Measured Ping between Bot and Discord",
                    format!("{} ms", (ts - t2).num_milliseconds()),
                    false,
                )
        })
    })
    .await?;

    Ok(())
}

async fn avatar(req: CommandData) -> Result<(), Error> {
    let mut user: Option<User> = None;
    if !req.parameters.is_empty() {
        let guild = req.message.guild_id;
        match guild {
            None => {
                user = req.parameters[0].as_discord_user(&req.ctx).await;
            }
            Some(guild) => {
                if let Some(u) = req.parameters[0]
                    .as_discord_guild_user(&req.ctx, &guild)
                    .await
                {
                    user = Some(u.user)
                }
            }
        }
    } else {
        user = Some(req.message.author);
    }
    if user.is_none() {
        req.message
            .channel_id
            .send_message(&req.ctx, |m| {
                m.embed(|e| setup_embed(e, "Avatar", "Can't find that user"))
            })
            .await?;
        return Ok(());
    }
    req.message
        .channel_id
        .send_message(&req.ctx, |m| {
            m.embed(|e| {
                set_default_style(e)
                    .title("Avatar")
                    .image(user.unwrap().avatar_url().unwrap())
            })
        })
        .await?;

    Ok(())
}
