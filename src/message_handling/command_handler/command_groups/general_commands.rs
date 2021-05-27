use super::super::command_builder::CommandBuilder;
use super::send_message;
use crate::embed::{set_default_embed_style, setup_embed};
use crate::message_handling::command_handler::command_data::CommandData;
use crate::message_handling::command_handler::command_groups::command_group::CommandGroup;
use crate::message_handling::command_handler::parameter_matcher::ParameterType;
use crate::message_handling::permission::PermissionLevel;
use serenity::model::user::User;
use serenity::Error;

lazy_static! {
    pub static ref GENERAL_COMMANDS: CommandGroup = {
        CommandGroup {
            name: "General".to_string(),
            commands: vec![
                CommandBuilder::new("help", PermissionLevel::Everyone)
                    .with_help("Displays a list of commands for the bot", 
r#"Allows you to see all commands you can use for your permission level, along with a description
Usage:
``help`` for a list of all commands useable for you.
``help`` [command name] for more detailed info on a specific command."#)
                    .with_func(Box::new(help))
                    .with_parameters(vec![ParameterType::Word])
                    .build(),
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
                CommandBuilder::new("whatdoyouthinkdeukbot", PermissionLevel::Everyone)
                    .with_alternative("whatdoyouthink")
                    .with_help("Gives the bots opinion about something", "Gives the bots opinion about something\nusage:\n``whatdoyouthink {about something}``")
                    .with_func(Box::new(bot_opinion))
                    .with_parameters(vec![ParameterType::Remainder])
                    .build(),
            ],
        }
    };
}

async fn help(req: CommandData) -> Result<(), Error> {
    send_message(&req.message.channel_id, &req.ctx, |m| {
        m.embed(|e| {
            set_default_embed_style(&req.current_user, e);
            if !req.parameters.is_empty() {
                let command_name = &req.parameters[0].value;
                let cmd =
                    super::COMMAND_LOOKUP.get(&unicase::UniCase::new(command_name.to_string()));
                match cmd {
                    None => {
                        e.title("Help");
                        e.description(format!("Unable to find command by name '{}'", command_name));
                    }
                    Some(cmd) => {
                        e.title(format!("{} Help", cmd.get_name()));
                        match cmd.get_long_help() {
                            None => {
                                e.description("This command does not have a description.");
                            }
                            Some(h) => {
                                e.description(h);
                            }
                        }
                    }
                }
            } else {
                super::build_help_function(e, req.permission);
            }
            e
        })
    })
    .await?;
    Ok(())
}

async fn info(req: CommandData) -> Result<(), Error> {
    send_message(&req.message.channel_id, &req.ctx, |m| {
        m.embed(|e| {
            setup_embed(
                &req.current_user,
                e,
                "Deukbot",
                "A bot designed by Deukhoofd",
            )
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
    let msg_promise = send_message(&req.message.channel_id, &req.ctx, |m| {
        m.embed(|e| {
            setup_embed(&req.current_user, e, "Pong", "Pong").field(
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
            setup_embed(&req.current_user, e, "Pong", "Pong")
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
        let u = &req.current_user;
        send_message(&req.message.channel_id, &req.ctx, |m| {
            m.embed(|e| setup_embed(u, e, "Avatar", "Can't find that user"))
        })
        .await?;
        return Ok(());
    }

    let u = &req.current_user;
    send_message(&req.message.channel_id, &req.ctx, |m| {
        m.embed(|e| {
            set_default_embed_style(u, e)
                .title("Avatar")
                .image(user.unwrap().face())
        })
    })
    .await?;

    Ok(())
}

async fn bot_opinion(req: CommandData) -> Result<(), Error> {
    let opinion = crate::utilities::bot_opinions::get_opinion(&req.parameters[0].value);
    send_message(&req.message.channel_id, &req.ctx, |m| {
        m.embed(|e| setup_embed(&req.current_user, e, "Opinion", opinion))
    })
    .await?;
    Ok(())
}
