//! tool for checking if a vote is valid

use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker, UserMarker},
    Id,
};

use crate::{client::bot::StarboardBot, core::emoji::SimpleEmoji};

use super::config::StarboardConfig;

#[derive(Debug)]
pub enum VoteStatus {
    Ignore,
    Remove,
    Valid((Vec<StarboardConfig>, Vec<StarboardConfig>)),
}

impl VoteStatus {
    pub async fn get_vote_status(
        _bot: &StarboardBot,
        emoji: &SimpleEmoji,
        configs: Vec<StarboardConfig>,
        reactor_id: Id<UserMarker>,
        _message_id: Id<MessageMarker>,
        _channel_id: Id<ChannelMarker>,
        message_author_id: Id<UserMarker>,
        message_author_is_bot: bool,
    ) -> VoteStatus {
        let mut invalid_exists = false;
        let mut allow_remove = true;

        let mut upvote = Vec::new();
        let mut downvote = Vec::new();
        for config in configs.into_iter() {
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
            let is_valid;

            // settings to check:
            // - self_vote (done)
            // - allow_bots (done)
            // - require_image
            // - older_than
            // - newer_than
            // alow need to check permroles

            if !config.resolved.self_vote && reactor_id == message_author_id {
                // self-vote
                is_valid = false;
            } else if !config.resolved.allow_bots && message_author_is_bot {
                // allow-bots
                is_valid = false;
            } else {
                is_valid = true;
            }

            if !is_valid {
                invalid_exists = true;
                continue;
            }

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
