use super::super::command_builder::CommandBuilder;
use super::super::command_group::CommandGroup;
use crate::message_handling::command_handler::command_data::CommandData;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::Error;

lazy_static! {
    pub static ref GENERAL_COMMANDS: CommandGroup = {
        CommandGroup {
            commands: vec![
                CommandBuilder::new("info")
                    .with_func(Box::new(info))
                    .build(),
                CommandBuilder::new("ping")
                    .with_func(Box::new(ping))
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
                crate::embed::set_default_style(e);
                e.title("Deukbot");
                e.description("A bot designed by Deukhoofd");
                e.field("Software", "Deukbot 5.0", true);
                e.field("Creator", "<@84372569012043776>", true);
                e
            });

            m
        })
        .await?;
    Ok(())
}

async fn ping(req: CommandData) -> Result<(), Error> {
    info!("PING!");
    req.message.reply(req.ctx, "Pong!").await?;
    Ok(())
}
