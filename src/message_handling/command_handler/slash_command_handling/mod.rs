mod global_commands;
pub mod slash_command;

use crate::embed::setup_embed;
use crate::message_handling::command_handler::slash_command_handling::slash_command::{
    InteractionData, InteractionFn,
};
use serenity::client::Context;
use serenity::model::id::{CommandId, GuildId};
use serenity::model::interactions::{Interaction, InteractionResponseType};
use slash_command::SlashCommand;
use std::collections::HashMap;
use std::error::Error;
use std::lazy::SyncLazy;
use tokio::sync::Mutex;

static GLOBAL_COMMANDS: SyncLazy<Vec<SlashCommand>> =
    SyncLazy::new(global_commands::create_global_commands);

static INTERACTION_LOOKUP: SyncLazy<
    Mutex<HashMap<CommandId, &'static Box<dyn InteractionFn + Send + Sync + 'static>>>,
> = SyncLazy::new(|| Mutex::new(HashMap::new()));

pub async fn register_global_commands(ctx: Context) -> Result<(), Box<dyn Error>> {
    for command in GLOBAL_COMMANDS.iter() {
        let res = GuildId(186120649603809280)
            .create_application_command(&ctx, &command.build_function)
            .await;
        if res.is_err() {
            error!(
                "Failed to create guild command: {}",
                res.err().unwrap().to_string()
            );
        } else {
            let cmd = res.unwrap();
            INTERACTION_LOOKUP
                .lock()
                .await
                .insert(cmd.id, &command.call_function);

            info!("Registered command by name '{}'", cmd.name);
        }
    }

    Ok(())
}

pub async fn call_command(ctx: Context, interaction: Interaction) -> Result<(), Box<dyn Error>> {
    let id = interaction.data.as_ref().unwrap().id;
    let lookup = INTERACTION_LOOKUP.lock().await;
    let cmd_opt = lookup.get(&id);
    let u = ctx.cache.current_user().await;
    let user_id = u.id.0;

    interaction
        .create_interaction_response(&ctx, |c| {
            c.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                .interaction_response_data(|m| m.content("Working..."))
        })
        .await?;

    match cmd_opt {
        None => {
            interaction
                .edit_original_interaction_response(&ctx, user_id, |c| {
                    c.create_embed(|e| setup_embed(&u, e, "Unknown Command", "Unknown Command"))
                })
                .await?;
        }
        Some(cmd) => {
            let ctx = Box::new(ctx);
            let interaction = Box::new(interaction);
            let u = Box::new(u);
            let interaction_data = InteractionData {
                ctx: ctx.clone(),
                req: interaction.clone(),
                user: u.clone(),
            };
            let mut errored: bool = false;
            {
                let res = cmd.call(interaction_data).await;
                if res.is_err() {
                    error!(
                        "Encountered an error with command '{}': '{}'",
                        interaction.data.as_ref().unwrap().name.to_string(),
                        res.as_ref().err().unwrap()
                    );
                    errored = true;
                }
            }
            if errored {
                let _ = interaction
                    .edit_original_interaction_response(&*ctx, user_id, |c| {
                        c.create_embed(|e| {
                            setup_embed(
                                &*u,
                                e,
                                "Encountered error",
                                "The given command caused an error. Please try again later.",
                            )
                        })
                    })
                    .await;
            }
        }
    }
    Ok(())
}
