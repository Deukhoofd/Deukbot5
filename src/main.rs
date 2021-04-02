#![feature(const_mut_refs)]
#![feature(fn_traits)]
#![feature(in_band_lifetimes)]

pub mod deukbot_result;
pub mod embed;
pub mod global;
pub mod message_handling;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};

#[macro_use]
extern crate log;
extern crate simplelog;

#[macro_use]
extern crate lazy_static;

use simplelog::*;

use crate::message_handling::handle_message;
use deukbot_result::DeukbotResult;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use std::env;

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
        global::set_deukbot_id(_data_about_bot.user.id);
        global::set_self_user(&_ctx).await;
    }
}

fn setup_logging() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        ConfigBuilder::new()
            .add_filter_allow_str("deukbot5")
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}

#[tokio::main]
async fn main() {
    setup_logging();

    info!("============================");
    info!("=== Starting Up Deukbot! ===");
    info!("============================");

    message_handling::command_handler::setup_commands();

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}