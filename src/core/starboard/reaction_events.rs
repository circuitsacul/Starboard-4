use twilight_model::gateway::payload::incoming::{
    ReactionAdd, ReactionRemove, ReactionRemoveAll, ReactionRemoveEmoji,
};

use crate::{
    client::bot::StarboardBot,
    core::emoji::SimpleEmoji,
    database::{Member, Message, User, Vote},
    map_dup_none, unwrap_id,
};

use super::{config::StarboardConfig, handle::RefreshMessage, vote_status::VoteStatus};

pub async fn handle_reaction_add(
    bot: &StarboardBot,
    event: Box<ReactionAdd>,
) -> anyhow::Result<()> {
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

    if !StarboardConfig::is_guild_vote_emoji(bot, unwrap_id!(guild_id), &emoji.raw).await? {
        return Ok(());
    }

    let orig_msg = Message::get_original(&bot.pool, unwrap_id!(event.message_id)).await?;
    let orig_msg = match orig_msg {
        None => {
            // author data
            let (author_is_bot, author_id) = {
                let orig_msg_obj = bot
                    .cache
                    .fog_message(&bot, event.channel_id, event.message_id)
                    .await?;
                let orig_msg_obj = match orig_msg_obj.value() {
                    None => return Ok(()),
                    Some(obj) => obj,
                };

                bot.cache.ensure_user(&bot, orig_msg_obj.author_id).await?;
                let cached_author_is_bot =
                    bot.cache.users.with(&orig_msg_obj.author_id, |_, user| {
                        user.as_ref().map(|u| u.is_bot)
                    });
                match cached_author_is_bot {
                    None => panic!("No cached user after ensuring."),
                    Some(is_bot) => (is_bot, unwrap_id!(orig_msg_obj.author_id)),
                }
            };

            map_dup_none!(User::create(&bot.pool, author_id, author_is_bot))?;
            map_dup_none!(Member::create(&bot.pool, author_id, unwrap_id!(guild_id)))?;

            let is_nsfw = bot
                .cache
                .fog_channel_nsfw(bot, guild_id, event.channel_id)
                .await?
                .unwrap();

            // message
            let orig = map_dup_none!(Message::create(
                &bot.pool,
                unwrap_id!(event.message_id),
                unwrap_id!(guild_id),
                unwrap_id!(event.channel_id),
                author_id,
                is_nsfw,
            ))?;

            match orig {
                Some(msg) => msg,
                None => Message::get(&bot.pool, unwrap_id!(event.message_id))
                    .await?
                    .unwrap(),
            }
        }
        Some(msg) => msg,
    };

    let configs = StarboardConfig::list_for_channel(bot, guild_id, event.channel_id).await?;
    let status =
        VoteStatus::get_vote_status(bot, &emoji, configs, event.message_id, event.channel_id).await;

    match status {
        VoteStatus::Ignore => Ok(()),
        VoteStatus::Remove => {
            let _ = bot
                .http
                .delete_reaction(
                    event.channel_id,
                    event.message_id,
                    &emoji.reactable(),
                    event.user_id,
                )
                .exec()
                .await;

            Ok(())
        }
        VoteStatus::Valid((upvote, downvote)) => {
            // create reactor data
            let reactor_user_id = unwrap_id!(reactor_member.user.id);
            map_dup_none!(User::create(
                &bot.pool,
                reactor_user_id,
                reactor_member.user.bot
            ))?;
            map_dup_none!(Member::create(
                &bot.pool,
                reactor_user_id,
                unwrap_id!(guild_id)
            ))?;

            // create the votes
            for config in upvote.iter() {
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
            for config in downvote.iter() {
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

            let all_configs: Vec<_> = upvote.into_iter().chain(downvote).collect();
            let mut refresh = RefreshMessage::new(&bot, event.message_id);
            refresh.set_configs(all_configs);
            refresh.set_sql_message(orig_msg);
            refresh.refresh().await?;

            Ok(())
        }
    }
}

pub async fn handle_reaction_remove(
    bot: &StarboardBot,
    event: Box<ReactionRemove>,
) -> anyhow::Result<()> {
    let guild_id = match event.guild_id {
        None => return Ok(()),
        Some(guild_id) => guild_id,
    };

    let orig = match Message::get_original(&bot.pool, unwrap_id!(event.message_id)).await? {
        None => return Ok(()),
        Some(orig) => orig,
    };

    let emoji = SimpleEmoji::from(event.emoji.clone());
    let configs = StarboardConfig::list_for_channel(bot, guild_id, event.channel_id).await?;
    let status =
        VoteStatus::get_vote_status(&bot, &emoji, configs, event.message_id, event.channel_id)
            .await;

    match status {
        VoteStatus::Valid((upvote, downvote)) => {
            let user_id = unwrap_id!(event.user_id);
            let all_configs: Vec<_> = upvote.into_iter().chain(downvote).collect();
            for config in all_configs.iter() {
                Vote::delete(&bot.pool, orig.message_id, config.starboard.id, user_id).await?;
            }

            let mut refresh = RefreshMessage::new(&bot, event.message_id);
            refresh.set_sql_message(orig);
            refresh.set_configs(all_configs);
            refresh.refresh().await?;

            Ok(())
        }
        VoteStatus::Ignore => Ok(()),
        VoteStatus::Remove => Ok(()),
    }
}

pub async fn handle_reaction_remove_all(
    _bot: &StarboardBot,
    _event: ReactionRemoveAll,
) -> anyhow::Result<()> {
    todo!()
}

pub async fn handle_reaction_remove_emoji(
    _bot: &StarboardBot,
    _event: ReactionRemoveEmoji,
) -> anyhow::Result<()> {
    todo!()
}
