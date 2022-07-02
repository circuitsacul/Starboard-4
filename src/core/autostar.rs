use std::time::Duration;

use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    client::bot::StarboardBot,
    core::emoji::{EmojiCommon, SimpleEmoji},
    database::AutoStarChannel,
    unwrap_id,
    utils::notify,
};

use super::has_image::has_image;

pub async fn handle(bot: StarboardBot, event: Box<MessageCreate>) -> anyhow::Result<()> {
    // Ignore DMs
    if event.guild_id.is_none() {
        return Ok(());
    }

    // Check the cache...
    if !bot.cache.autostar_channel_ids.contains(&event.channel_id) {
        return Ok(());
    }

    // Fetch the autostar channels
    let asc = AutoStarChannel::list_by_channel(&bot.pool, unwrap_id!(event.channel_id)).await?;

    // If none, remove the channel id from the cache
    if asc.len() == 0 {
        bot.cache.autostar_channel_ids.remove(&event.channel_id);
        return Ok(());
    }

    // Handle the autostar channels
    for a in asc.into_iter() {
        let status = get_status(&bot, &a, &event).await;

        if matches!(status, Status::InvalidStay) {
            continue;
        }
        if let Status::InvalidRemove(reasons) = status {
            let _ = bot
                .http
                .delete_message(event.channel_id, event.id)
                .exec()
                .await;

            if !event.author.bot {
                let message = {
                    format!(
                        "Your message <#{}> was deleted for the following reason(s):\n",
                        event.channel_id
                    ) + &reasons.join("\n - ")
                };
                notify::notify(&bot, event.author.id, &message).await;
            }

            continue;
        }

        for emoji in Vec::<SimpleEmoji>::from_stored(a.emojis) {
            let _ = bot
                .http
                .create_reaction(event.channel_id, event.id, &emoji.reactable())
                .exec()
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
    event: &Box<MessageCreate>,
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
                " - Your message cannot be longer than {} characters.",
                max_chars,
            ));
        }
    }
    if asc.require_image {
        if !has_image(&event.embeds, &event.attachments) {
            tokio::time::sleep(Duration::from_secs(3)).await;

            let updated_msg = bot.cache.messages.get(&event.id);
            let mut still_invalid = true;

            if let Some(msg) = updated_msg {
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
    }

    if invalid.is_empty() {
        Status::Valid
    } else if asc.delete_invalid {
        Status::InvalidRemove(invalid)
    } else {
        Status::InvalidStay
    }
}
