use std::sync::Arc;

use twilight_model::{
    gateway::payload::incoming::MessageUpdate,
    id::{marker::MessageMarker, Id},
};

use crate::{
    client::bot::StarboardBot,
    core::premium::is_premium::is_guild_premium,
    database::{DbMessage, Starboard, StarboardMessage, StarboardOverride},
    errors::StarboardResult,
    utils::{id_as_i64::GetI64, into_id::IntoId},
};

use super::{config::StarboardConfig, handle::RefreshMessage};

pub async fn handle_message_update(
    bot: Arc<StarboardBot>,
    event: Box<MessageUpdate>,
) -> StarboardResult<()> {
    let msg = match DbMessage::get(&bot.pool, event.id.get_i64()).await? {
        Some(msg) => msg,
        None => return Ok(()),
    };

    let is_premium = is_guild_premium(&bot, msg.guild_id).await?;
    let mut refresh = RefreshMessage::new(bot, event.id, is_premium);
    refresh.set_sql_message(msg);
    refresh.refresh(true).await?;

    Ok(())
}

pub async fn handle_message_delete(
    bot: Arc<StarboardBot>,
    message_id: Id<MessageMarker>,
) -> StarboardResult<()> {
    if bot.cache.auto_deleted_posts.get(&message_id).is_some() {
        bot.cache.auto_deleted_posts.invalidate(&message_id).await;
        return Ok(());
    }

    let message_id_i64 = message_id.get_i64();
    let msg = match DbMessage::get_original(&bot.pool, message_id_i64).await? {
        Some(msg) => msg,
        None => return Ok(()),
    };

    let must_force = 'out: {
        if message_id == msg.message_id {
            break 'out false;
        }

        // this means that a starboard message was deleted, so we want to remove that
        // from the database so that the affected starboard can resend it without
        // needing force=true
        let Some(sb_msg) = StarboardMessage::delete(&bot.pool, message_id_i64).await? else {
            break 'out false;
        };

        // handle the `on-delete` setting for the corresponding starboard
        let Some(sb) = Starboard::get(&bot.pool, sb_msg.starboard_id).await? else {
            break 'out false;
        };

        let guild_id = msg.guild_id.into_id();
        let channel_id = msg.channel_id.into_id();

        let channel_ids = bot
            .cache
            .qualified_channel_ids(&bot, guild_id, channel_id)
            .await?;
        let channel_ids = channel_ids
            .into_iter()
            .map(|id| id.get_i64())
            .collect::<Vec<_>>();
        let overrides =
            StarboardOverride::list_by_starboard_and_channels(&bot.pool, sb.id, &channel_ids)
                .await?;

        let config = StarboardConfig::new(sb, &channel_ids, overrides)?;

        match config.resolved.on_delete {
            0 => false,         // refresh
            1 => return Ok(()), // ignore
            2 => {
                DbMessage::set_trashed(
                    &bot.pool,
                    msg.message_id,
                    true,
                    Some("on-delete is set to Trash All, and this message was manually deleted."),
                )
                .await?;
                true
            }
            3 => {
                DbMessage::set_freeze(&bot.pool, msg.message_id, true).await?;
                true
            }
            _ => unreachable!("Invalid on-delete value."),
        }
    };

    let is_premium = is_guild_premium(&bot, msg.guild_id).await?;
    let mut refresh = RefreshMessage::new(bot, msg.message_id.into_id(), is_premium);
    if !must_force {
        refresh.set_sql_message(msg);
    }
    refresh.refresh(must_force).await?;

    Ok(())
}
