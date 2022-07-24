use twilight_model::gateway::payload::incoming::{
    ReactionAdd, ReactionRemove, ReactionRemoveAll, ReactionRemoveEmoji,
};

use crate::{client::bot::StarboardBot, core::emoji::SimpleEmoji};

use super::{config::StarboardConfig, vote_status::VoteStatus};

pub async fn handle_reaction_add(
    bot: &StarboardBot,
    event: Box<ReactionAdd>,
) -> anyhow::Result<()> {
    let guild_id = match event.guild_id {
        None => return Ok(()),
        Some(guild_id) => guild_id,
    };

    let emoji = SimpleEmoji::from(event.emoji.clone());
    let configs = StarboardConfig::list_for_channel(bot, guild_id, event.channel_id).await?;
    let status =
        VoteStatus::get_vote_status(bot, &emoji, &configs, event.message_id, event.channel_id)
            .await;

    println!("{:?}", status);
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
        VoteStatus::Valid((_upvote, _downvote)) => Ok(()),
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
