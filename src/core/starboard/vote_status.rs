//! tool for checking if a vote is valid

use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};

use crate::{client::bot::StarboardBot, core::emoji::SimpleEmoji};

use super::config::StarboardConfig;

#[derive(Debug)]
pub enum VoteStatus<'a> {
    Ignore,
    Remove,
    Valid((Vec<&'a StarboardConfig>, Vec<&'a StarboardConfig>)),
}

impl VoteStatus<'_> {
    pub async fn get_vote_status<'a>(
        _bot: &StarboardBot,
        emoji: &SimpleEmoji,
        configs: &'a Vec<StarboardConfig>,
        _message_id: Id<MessageMarker>,
        _channel_id: Id<ChannelMarker>,
    ) -> VoteStatus<'a> {
        let mut invalid_exists = false;
        let mut allow_remove = true;

        let mut upvote = Vec::new();
        let mut downvote = Vec::new();
        for config in configs.iter() {
            // skip disabled configurations
            if !config.resolved.enabled || config.starboard.premium_locked {
                continue;
            }

            let is_downvote;
            if config.resolved.upvote_emojis.contains(&emoji.raw) {
                is_downvote = false;
            } else if config.resolved.downvote_emojis.contains(&emoji.raw) {
                is_downvote = true;
            } else {
                continue;
            }

            // respect the `remove_invalid_reactions` setting
            if !config.resolved.remove_invalid_reactions {
                allow_remove = false;
            }

            // check settings
            // TODO

            // add to corresponding list
            if is_downvote {
                downvote.push(config)
            } else {
                upvote.push(config)
            }
        }
        if upvote.is_empty() && downvote.is_empty() {
            if invalid_exists && allow_remove {
                VoteStatus::Remove
            } else {
                VoteStatus::Ignore
            }
        } else {
            VoteStatus::Valid((upvote, downvote))
        }
    }
}
