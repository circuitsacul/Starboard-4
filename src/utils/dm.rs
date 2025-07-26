use twilight_http::request::channel::message::CreateMessage;
use twilight_model::id::{Id, marker::UserMarker};

use crate::{client::bot::StarboardBot, errors::StarboardResult};

pub async fn dm(
    bot: &'_ StarboardBot,
    user_id: Id<UserMarker>,
) -> StarboardResult<CreateMessage<'_>> {
    let dm_channel = bot
        .http
        .create_private_channel(user_id)
        .await?
        .model()
        .await?
        .id;
    Ok(bot.http.create_message(dm_channel))
}
