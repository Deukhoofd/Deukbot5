#![feature(const_mut_refs)]
#![feature(fn_traits)]
#![feature(in_band_lifetimes)]
#![feature(once_cell)]

#[allow(clippy::too_many_arguments)]
pub mod database;
pub mod defines;
pub mod deukbot_result;
pub mod embed;
pub mod global;
pub mod message_handling;
pub mod utilities;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};

#[macro_use]
extern crate log;
extern crate simplelog;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate enum_display_derive;
#[macro_use]
extern crate num_derive;

use simplelog::*;

use crate::message_handling::handle_message;
use deukbot_result::DeukbotResult;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::model::interactions::Interaction;
use std::env;

#[tokio::main]
async fn main() {
    setup_logging();

    info!("============================");
    info!("=== Starting Up Deukbot! ===");
    info!("============================");

    database::database_initialization::initialise_tables().await;

    message_handling::command_handler::command_groups::setup_commands();
    match env::var("OWNER_ID") {
        Ok(v) => match v.parse::<u64>() {
            Ok(owner_id) => global::set_owner_id(UserId(owner_id)),
            Err(_) => {
                error!("Given owner id was not an u64");
            }
        },
        Err(_) => {
            warn!("Owner ID was not set.");
        }
    }

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(835520127368691723)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start_autosharded().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, _new_message: Message) {
        let res = handle_message(_ctx, _new_message).await;
        if let DeukbotResult::Err(e) = res {
            error!("{}", e)
        }
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        info!("Ready!");
        crate::message_handling::command_handler::slash_command_handling::register_global_commands(
            _ctx,
        )
        .await
        .unwrap();
    }

    async fn interaction_create(&self, _ctx: Context, _interaction: Interaction) {
        let res = crate::message_handling::command_handler::slash_command_handling::call_command(
            _ctx,
            _interaction,
        )
        .await;
        if res.is_err() {
            error!(
                "Interaction response errored with: '{}'",
                res.err().unwrap().to_string()
            );
        }
    }
}

fn setup_logging() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Trace,
        ConfigBuilder::new()
            .add_filter_allow_str("deukbot5")
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}
