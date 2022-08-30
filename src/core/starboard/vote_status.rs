//! tool for checking if a vote is valid

use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker, UserMarker},
    Id,
};

use crate::{
    client::bot::StarboardBot,
    core::{emoji::SimpleEmoji, has_image::has_image},
    errors::StarboardResult,
};

use super::config::StarboardConfig;

#[derive(Debug)]
pub enum VoteStatus {
    Ignore,
    Remove,
    Valid((Vec<StarboardConfig>, Vec<StarboardConfig>)),
}

impl VoteStatus {
    pub async fn get_vote_status(
        bot: &StarboardBot,
        emoji: &SimpleEmoji,
        configs: Vec<StarboardConfig>,
        reactor_id: Id<UserMarker>,
        message_id: Id<MessageMarker>,
        channel_id: Id<ChannelMarker>,
        message_author_id: Id<UserMarker>,
        message_author_is_bot: bool,
        message_has_image: Option<bool>,
    ) -> StarboardResult<VoteStatus> {
        let message_has_image = match message_has_image {
            Some(val) => Some(val),
            None => match bot
                .cache
                .fog_message(&bot, channel_id, message_id)
                .await?
                .value()
            {
                Some(msg) => Some(has_image(&msg.embeds, &msg.attachments)),
                None => None,
            },
        };

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
            } else if config.resolved.require_image && !matches!(message_has_image, Some(true)) {
                // require-image
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
                Ok(VoteStatus::Remove)
            } else {
                Ok(VoteStatus::Ignore)
            }
        } else {
            Ok(VoteStatus::Valid((upvote, downvote)))
        }
    }
}
