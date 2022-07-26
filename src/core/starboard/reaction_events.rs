use twilight_model::gateway::payload::incoming::{
    ReactionAdd, ReactionRemove, ReactionRemoveAll, ReactionRemoveEmoji,
};

use crate::{
    client::bot::StarboardBot,
    core::emoji::SimpleEmoji,
    database::{Member, Message, User, Vote},
    map_dup_none, unwrap_id,
};

use super::{config::StarboardConfig, vote_status::VoteStatus};

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

    let orig_msg = Message::get_original(&bot.pool, unwrap_id!(event.message_id)).await?;
    let orig_msg = match orig_msg {
        None => {
            // author data
            let (author_is_bot, author_id) = {
                let orig_msg_obj = bot
                    .cache
                    .fog_message(&bot, event.channel_id, event.message_id)
                    .await;
                let (author_is_bot, author_id) = bot
                    .cache
                    .users
                    .with(&orig_msg_obj.value().author_id, |author_id, user| {
                        (user.as_ref().map(|u| u.is_bot), unwrap_id!(author_id))
                    });
                match author_is_bot {
                    None => return Ok(()),
                    Some(is_bot) => (is_bot, author_id),
                }
            };

            map_dup_none!(User::create(&bot.pool, author_id, author_is_bot))?;
            map_dup_none!(Member::create(&bot.pool, author_id, unwrap_id!(guild_id)))?;

            // all this work just to check if a channel/thread is nsfw
            // thanks discord
            let is_nsfw = {
                // first, we need to fetch the channel from discord.
                let channel = bot
                    .http
                    .channel(event.channel_id)
                    .exec()
                    .await?
                    .model()
                    .await?;

                // if by some miracle nsfw is Some...
                if let Some(nsfw) = channel.nsfw {
                    nsfw
                } else {
                    // hopefully it's because this is a thread
                    if !channel.kind.is_thread() {
                        // not much we can do at this point really
                        panic!("Non-thread channel had no `nsfw` parameter.");
                    }

                    // is a thread, should have a parent_id
                    // yes we have to make another fetch
                    // don't you just love discord sometimes
                    let parent = bot
                        .http
                        .channel(channel.parent_id.unwrap())
                        .exec()
                        .await?
                        .model()
                        .await?;
                    if let Some(nsfw) = parent.nsfw {
                        nsfw
                    } else {
                        // either a major bug, or discord pushed a breaking api change
                        // probably both
                        panic!("Parent of thread had no `nsfw` parameter.");
                    }
                }
            };

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
                None => Message::get_original(&bot.pool, unwrap_id!(event.message_id))
                    .await?
                    .unwrap(),
            }
        }
        Some(msg) => msg,
    };

    let emoji = SimpleEmoji::from(event.emoji.clone());
    let configs = StarboardConfig::list_for_channel(bot, guild_id, event.channel_id).await?;
    let status =
        VoteStatus::get_vote_status(bot, &emoji, &configs, event.message_id, event.channel_id)
            .await;

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
            for config in upvote {
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
            for config in downvote {
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

            Ok(())
        }
    }
}

pub async fn handle_reaction_remove(
    _bot: &StarboardBot,
    _event: Box<ReactionRemove>,
) -> anyhow::Result<()> {
    todo!()
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
