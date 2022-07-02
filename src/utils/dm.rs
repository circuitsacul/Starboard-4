use twilight_http::request::channel::message::CreateMessage;
use twilight_model::id::{marker::UserMarker, Id};

use crate::client::bot::StarboardBot;

pub async fn dm(
    bot: &StarboardBot,
    user_id: Id<UserMarker>,
) -> Result<CreateMessage, twilight_http::Error> {
    let dm_channel = bot
        .http
        .create_private_channel(user_id)
        .exec()
        .await?
        .model()
        .await
        .unwrap()
        .id;
    Ok(bot.http.create_message(dm_channel))
}
