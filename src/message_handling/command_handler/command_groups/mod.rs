use serenity::builder::CreateMessage;
use serenity::http::Http;
use serenity::model::prelude::{ChannelId, Message};

pub mod general_commands;

pub async fn send_message<'a, F>(
    channel_id: &ChannelId,
    http: impl AsRef<Http>,
    f: F,
) -> Result<Message, serenity::Error>
where
    for<'b> F: FnOnce(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a>,
{
    let start_time = chrono::Utc::now();
    let res = channel_id.send_message(http, f).await;
    trace!(
        "Sending message took: {} ms",
        ((chrono::Utc::now() - start_time).num_milliseconds())
    );
    res
}
