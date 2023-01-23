use std::sync::Arc;

use twilight_model::gateway::payload::incoming::{ReactionAdd, ReactionRemove};

use crate::{
    client::bot::StarboardBot,
    core::{emoji::SimpleEmoji, stats::refresh_xp},
    database::{DbMember, DbMessage, DbUser, Vote},
    errors::StarboardResult,
    utils::{id_as_i64::GetI64, into_id::IntoId},
};

use super::{
    config::StarboardConfig,
    handle::RefreshMessage,
    vote_status::{VoteContext, VoteStatus},
};

pub async fn handle_reaction_add(
    bot: Arc<StarboardBot>,
    event: Box<ReactionAdd>,
) -> StarboardResult<()> {
    let guild_id = match event.guild_id {
        None => return Ok(()),
        Some(guild_id) => guild_id,
    };
    if !bot.cache.guilds.contains_key(&guild_id) {
        return Ok(());
    }
    let reactor_member = event
        .member
        .as_ref()
        .expect("No member object in reaction_add");
    if reactor_member.user.bot {
        return Ok(());
    }

    let emoji = SimpleEmoji::from(event.emoji.clone());

    if !StarboardConfig::is_guild_vote_emoji(&bot, guild_id.get_i64(), &emoji.raw).await? {
        return Ok(());
    }

    let orig_msg = DbMessage::get_original(&bot.pool, event.message_id.get_i64()).await?;
    let (orig_msg, author_is_bot) = match orig_msg {
        None => {
            // author data
            let (author_is_bot, author_id) = {
                let orig_msg_obj = bot
                    .cache
                    .fog_message(&bot, event.channel_id, event.message_id)
                    .await?;
                let orig_msg_obj = match orig_msg_obj {
                    None => return Ok(()),
                    Some(obj) => obj,
                };

                let user = bot.cache.fog_user(&bot, orig_msg_obj.author_id).await?;
                let is_bot = user.map(|u| u.is_bot).unwrap_or(false);
                (is_bot, orig_msg_obj.author_id.get_i64())
            };

            DbUser::create(&bot.pool, author_id, author_is_bot).await?;
            DbMember::create(&bot.pool, author_id, guild_id.get_i64()).await?;

            let is_nsfw = bot
                .cache
                .fog_channel_nsfw(&bot, guild_id, event.channel_id)
                .await?
                .unwrap();

            // message
            let orig = DbMessage::create(
                &bot.pool,
                event.message_id.get_i64(),
                guild_id.get_i64(),
                event.channel_id.get_i64(),
                author_id,
                is_nsfw,
            )
            .await?;

            match orig {
                Some(msg) => (msg, author_is_bot),
                None => {
                    let msg = DbMessage::get(&bot.pool, event.message_id.get_i64())
                        .await?
                        .unwrap();
                    (msg, author_is_bot)
                }
            }
        }
        Some(msg) => {
            let user = DbUser::get(&bot.pool, msg.author_id).await?.unwrap();
            (msg, user.is_bot)
        }
    };

    let configs = StarboardConfig::list_for_channel(&bot, guild_id, event.channel_id).await?;
    let vote = VoteContext {
        emoji: &emoji,
        reactor_id: event.user_id,
        message_id: orig_msg.message_id.into_id(),
        channel_id: orig_msg.channel_id.into_id(),
        message_author_id: orig_msg.author_id.into_id(),
        message_author_is_bot: author_is_bot,
        message_has_image: None,
        message_is_frozen: orig_msg.frozen,
    };
    let status = VoteStatus::get_vote_status(&bot, vote, &configs).await?;

    // for future user, since orig_msg is moved
    let author_id = orig_msg.author_id;

    match status {
        VoteStatus::Ignore => (),
        VoteStatus::Remove => {
            let _ = bot
                .http
                .delete_reaction(
                    event.channel_id,
                    event.message_id,
                    &emoji.reactable(),
                    event.user_id,
                )
                .await;
        }
        VoteStatus::Valid((upvote, downvote)) => {
            // create reactor data
            let reactor_user_id = reactor_member.user.id.get_i64();
            DbUser::create(&bot.pool, reactor_user_id, reactor_member.user.bot).await?;
            DbMember::create(&bot.pool, reactor_user_id, guild_id.get_i64()).await?;

            // create the votes
            for config in &upvote {
                Vote::create(
                    &bot.pool,
                    orig_msg.message_id,
                    config.starboard.id,
                    reactor_user_id,
                    orig_msg.author_id,
                    false,
                )
                .await?;
            }
            for config in &downvote {
                Vote::create(
                    &bot.pool,
                    orig_msg.message_id,
                    config.starboard.id,
                    reactor_user_id,
                    orig_msg.author_id,
                    true,
                )
                .await?;
            }

            let mut refresh = RefreshMessage::new(bot.clone(), event.message_id);
            refresh.set_configs(configs.into_iter().map(Arc::new).collect());
            refresh.set_sql_message(orig_msg);
            refresh.refresh(false).await?;
        }
    }

    refresh_xp(&bot, guild_id, author_id.into_id()).await?;

    Ok(())
}

pub async fn handle_reaction_remove(
    bot: Arc<StarboardBot>,
    event: Box<ReactionRemove>,
) -> StarboardResult<()> {
    let guild_id = match event.guild_id {
        None => return Ok(()),
        Some(guild_id) => guild_id,
    };

    let orig = match DbMessage::get_original(&bot.pool, event.message_id.get_i64()).await? {
        None => return Ok(()),
        Some(orig) => orig,
    };
    let author = DbUser::get(&bot.pool, orig.author_id).await?.unwrap();

    let emoji = SimpleEmoji::from(event.emoji.clone());
    let configs = StarboardConfig::list_for_channel(&bot, guild_id, event.channel_id).await?;
    let vote = VoteContext {
        emoji: &emoji,
        reactor_id: event.user_id,
        message_id: orig.message_id.into_id(),
        channel_id: orig.channel_id.into_id(),
        message_author_id: orig.author_id.into_id(),
        message_author_is_bot: author.is_bot,
        message_has_image: None,
        message_is_frozen: orig.frozen,
    };
    let status = VoteStatus::get_vote_status(&bot, vote, &configs).await?;

    match status {
        VoteStatus::Valid((upvote, downvote)) => {
            let user_id = event.user_id.get_i64();
            let all_configs: Vec<_> = upvote.into_iter().chain(downvote).collect();
            for config in &all_configs {
                Vote::delete(&bot.pool, orig.message_id, config.starboard.id, user_id).await?;
            }

            let mut refresh = RefreshMessage::new(bot.clone(), event.message_id);
            refresh.set_sql_message(orig);
            refresh.set_configs(configs.into_iter().map(Arc::new).collect());
            refresh.refresh(false).await?;
        }
        VoteStatus::Ignore | VoteStatus::Remove => (),
    }

    refresh_xp(&bot, guild_id, author.user_id.into_id()).await?;

    Ok(())
}
