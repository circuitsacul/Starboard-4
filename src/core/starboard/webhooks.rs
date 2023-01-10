use std::sync::Arc;

use twilight_model::channel::Webhook;

use crate::{
    client::bot::StarboardBot,
    database::Starboard,
    errors::StarboardResult,
    utils::{id_as_i64::GetI64, into_id::IntoId},
};

pub async fn get_valid_webhook(
    bot: &StarboardBot,
    starboard: &Starboard,
    allow_create: bool,
) -> StarboardResult<Option<Arc<Webhook>>> {
    if let Some(wh) = starboard.webhook_id {
        if let Some(wh) = bot.cache.fog_webhook(bot, wh.into_id()).await? {
            return Ok(Some(wh));
        }

        Starboard::set_webhook(&bot.pool, starboard.id, None).await?;
    }

    if !allow_create {
        return Ok(None);
    }

    let name = format!("Webhook for '{}'", starboard.name);
    let wh = bot
        .http
        .create_webhook(starboard.channel_id.into_id(), &name)?;

    let Ok(wh) = wh.await else {
        return Ok(None);
    };
    let wh = Arc::new(wh.model().await?);

    bot.cache.webhooks.insert(wh.id, wh.clone());

    Starboard::set_webhook(&bot.pool, starboard.id, Some(wh.id.get_i64())).await?;

    Ok(Some(wh))
}
