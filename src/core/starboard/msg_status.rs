use crate::{
    client::bot::StarboardBot, database::Message as DbMessage, errors::StarboardResult,
    utils::into_id::IntoId,
};

use super::config::StarboardConfig;

#[derive(Debug)]
pub enum MessageStatus {
    /// true -> full update, false -> partial update
    Update(bool),
    Remove,
    /// true -> full update, false -> partial update
    Send(bool),
}

pub async fn get_message_status(
    bot: &StarboardBot,
    starboard_config: &StarboardConfig,
    message: &DbMessage,
    deleted: bool,
    points: i32,
) -> StarboardResult<MessageStatus> {
    let guild_id = starboard_config.starboard.guild_id.into_id();
    let sb_channel_id = starboard_config.starboard.channel_id.into_id();
    let sb_is_nsfw = bot
        .cache
        .fog_channel_nsfw(bot, guild_id, sb_channel_id)
        .await?;

    let sb_is_nsfw = match sb_is_nsfw {
        Some(val) => val,
        None => return Ok(MessageStatus::Update(starboard_config.resolved.link_edits)),
    };

    if (deleted && starboard_config.resolved.link_deletes)
        || (message.is_nsfw && !sb_is_nsfw)
        || message.trashed
    {
        Ok(MessageStatus::Remove)
    } else if message.forced_to.contains(&starboard_config.starboard.id) {
        Ok(MessageStatus::Send(starboard_config.resolved.link_edits))
    } else if message.frozen {
        Ok(MessageStatus::Update(false))
    } else if points >= starboard_config.resolved.required as _ {
        Ok(MessageStatus::Send(starboard_config.resolved.link_edits))
    } else if points <= starboard_config.resolved.required_remove as _ {
        Ok(MessageStatus::Remove)
    } else {
        Ok(MessageStatus::Update(starboard_config.resolved.link_edits))
    }
}
