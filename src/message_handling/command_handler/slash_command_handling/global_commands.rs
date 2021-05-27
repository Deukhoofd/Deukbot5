use crate::embed::{set_default_embed_style, setup_embed};
use crate::message_handling::command_handler::slash_command_handling::slash_command::{
    InteractionData, SlashCommand,
};
use serenity::model::interactions::ApplicationCommandInteractionDataOptionValue;
use serenity::model::prelude::{ApplicationCommandOptionType, Interaction, User};
use std::error::Error;

pub fn create_global_commands() -> Vec<SlashCommand> {
    vec![
        SlashCommand {
            build_function: Box::new(|a| a.name("ping").description("Sends a pong response")),
            call_function: Box::new(ping),
        },
        SlashCommand {
            build_function: Box::new(|a| {
                a.name("avatar")
                    .description("Gets a users avatar. Returns the avatar of the user using the command if no user was specified.")
                    .create_option(|o| {
                        o.name("user")
                            .description("The user to get the avatar of.")
                            .kind(ApplicationCommandOptionType::User)
                            .required(false)
                    })
            }),
            call_function: Box::new(avatar),
        },
        SlashCommand {
            build_function: Box::new(|a| {
                a.name("whatdoyouthink")
                    .description("Gets the bots opinion on something.")
                    .create_option(|o| {
                        o.name("subject")
                            .description("The subject to give an opinion about.")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
            }),
            call_function: Box::new(bot_opinion),
        },
    ]
}

fn get_user(interaction: &Interaction) -> &User {
    match &interaction.member {
        None => &interaction.user.as_ref().unwrap(),
        Some(member) => &member.user,
    }
}

async fn ping(data: InteractionData) -> Result<(), Box<dyn Error>> {
    data.req
        .edit_original_interaction_response(&*data.ctx, data.user.id.0, |c| {
            c.create_embed(|e| setup_embed(&data.user, e, "Pong", "Pong"))
        })
        .await?;
    Ok(())
}

async fn avatar(req: InteractionData) -> Result<(), Box<dyn Error>> {
    let interaction = &req.req;
    let user: &User = match interaction.data {
        None => (get_user(interaction)),
        Some(ref data) => {
            if data.options.is_empty() {
                get_user(interaction)
            } else {
                match data.options.first().as_ref() {
                    None => get_user(interaction),
                    Some(r) => match &r.resolved {
                        Some(ApplicationCommandInteractionDataOptionValue::User(u, ..)) => u,
                        _ => get_user(interaction),
                    },
                }
            }
        }
    };
    interaction
        .edit_original_interaction_response(&*req.ctx, req.user.id.0, |c| {
            c.create_embed(|e| {
                set_default_embed_style(&req.user, e)
                    .title("Avatar")
                    .image(user.face())
            })
        })
        .await?;
    Ok(())
}

async fn bot_opinion(req: InteractionData) -> Result<(), Box<dyn Error>> {
    match req.req.data {
        None => {
            return Ok(());
        }
        Some(ref data) => {
            if data.options.is_empty() {
                return Ok(());
            } else {
                match &data.options.first().as_ref().unwrap().value {
                    None => return Ok(()),
                    Some(bod) => {
                        let opinion = crate::utilities::bot_opinions::get_opinion(&bod.to_string());
                        req.req
                            .edit_original_interaction_response(&*req.ctx, req.user.id.0, |c| {
                                c.create_embed(|e| setup_embed(&req.user, e, "Opinion", opinion))
                            })
                            .await?;
                    }
                }
            }
        }
    }

    Ok(())
}
