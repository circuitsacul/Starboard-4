use std::{sync::Arc, time::Duration};

use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};

use crate::{
    cache::models::message::CachedMessage,
    client::bot::StarboardBot,
    core::emoji::{EmojiCommon, SimpleEmoji},
    database::AutoStarChannel,
    errors::StarboardResult,
    utils::{id_as_i64::GetI64, notify},
};

use super::has_image::has_image;

pub async fn handle(
    bot: &StarboardBot,
    autostar_channel_id: Id<ChannelMarker>,
    channel_id: Id<ChannelMarker>,
    message_id: Id<MessageMarker>,
    message: Option<&CachedMessage>,
) -> StarboardResult<()> {
    // Check the cache...
    if !bot
        .cache
        .autostar_channel_ids
        .contains(&autostar_channel_id)
    {
        return Ok(());
    }

    // Check cooldown
    if bot
        .cooldowns
        .autostar_send
        .trigger(&autostar_channel_id)
        .is_some()
    {
        return Ok(());
    }

    // Fetch the autostar channels
    let asc = AutoStarChannel::list_by_channel(&bot.pool, autostar_channel_id.get_i64()).await?;
    let asc: Vec<_> = asc.into_iter().filter(|a| !a.premium_locked).collect();

    // If none, remove the channel id from the cache
    if asc.is_empty() {
        bot.cache.autostar_channel_ids.remove(&autostar_channel_id);
        return Ok(());
    }

    let message_owner: Arc<CachedMessage>;
    let message = match message {
        Some(msg) => msg,
        None => {
            let Some(msg) = bot.cache.fog_message(bot, channel_id, message_id).await? else {
                return Ok(());
            };
            message_owner = msg;
            &message_owner
        }
    };

    // Handle the autostar channels
    for a in asc {
        let status = get_status(bot, &a, message_id, message).await;

        if matches!(status, Status::InvalidStay) {
            continue;
        }
        if let Status::InvalidRemove(reasons) = status {
            let _ = bot.http.delete_message(channel_id, message_id).await;

            let send = bot
                .cache
                .fog_user(bot, message.author_id)
                .await?
                .map_or(false, |u| !u.is_bot);
            if send {
                let to_send = {
                    format!(
                        "Your message in <#{channel_id}> was deleted for the following reason(s):\n"
                    ) + &reasons.join("\n - ")
                };
                notify::notify(bot, message.author_id, &to_send).await?;
            }

            continue;
        }

        for emoji in Vec::<SimpleEmoji>::from_stored(a.emojis) {
            let _ = bot
                .http
                .create_reaction(channel_id, message_id, &emoji.reactable())
                .await;
        }
    }

    Ok(())
}

enum Status {
    Valid,
    InvalidStay,
    InvalidRemove(Vec<String>),
}

async fn get_status(
    bot: &StarboardBot,
    asc: &AutoStarChannel,
    message_id: Id<MessageMarker>,
    event: &CachedMessage,
) -> Status {
    let mut invalid = Vec::new();

    if asc.min_chars != 0 && event.content.len() < asc.min_chars as usize {
        invalid.push(format!(
            " - Your message must have at least {} characters.",
            asc.min_chars
        ));
    }
    if let Some(max_chars) = asc.max_chars {
        if event.content.len() > max_chars as usize {
            invalid.push(format!(
                " - Your message cannot be longer than {max_chars} characters.",
            ));
        }
    }
    if asc.require_image && !has_image(&event.embeds, &event.attachments) {
        tokio::time::sleep(Duration::from_secs(3)).await;

        let updated_msg = bot.cache.messages.get(&message_id);
        let mut still_invalid = true;

        if let Some(msg) = updated_msg {
            let msg = match msg.value() {
                None => return Status::InvalidStay,
                Some(msg) => msg,
            };
            if has_image(&msg.embeds, &msg.attachments) {
                still_invalid = false;
            }
        } else {
            eprintln!(concat!(
                "Warning: autostar channel message was not cached. Likely means the cache is ",
                "being overwhelmed."
            ))
        }

        if still_invalid {
            invalid.push(" - Your message must include an image.".to_string());
        }
    }

    if invalid.is_empty() {
        Status::Valid
    } else if asc.delete_invalid {
        Status::InvalidRemove(invalid)
    } else {
        Status::InvalidStay
    }
}
