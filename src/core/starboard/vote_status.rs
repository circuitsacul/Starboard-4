//! tool for checking if a vote is valid

use std::time::{SystemTime, UNIX_EPOCH};

use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker, UserMarker},
    Id,
};
use twilight_util::snowflake::Snowflake;

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
            None => match &*bot.cache.fog_message(&bot, channel_id, message_id).await? {
                Some(msg) => Some(has_image(&msg.embeds, &msg.attachments)),
                None => None,
            },
        };

        let mut invalid_exists = false;
        let mut allow_remove = true;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        #[derive(Clone, Copy, PartialEq, Eq)]
        enum VoteType {
            Upvote,
            Downvote,
        }

        let eval_config = |config: StarboardConfig| {
            // skip disabled configurations
            if !config.resolved.enabled || config.starboard.premium_locked {
                return None;
            }

            let vote_type = if config.resolved.upvote_emojis.contains(&emoji.raw) {
                VoteType::Upvote
            } else if config.resolved.downvote_emojis.contains(&emoji.raw) {
                VoteType::Downvote
            } else {
                return None;
            };

            // respect the `remove_invalid_reactions` setting
            if !config.resolved.remove_invalid_reactions {
                allow_remove = false;
            }

            // message age in seconds
            let message_age: i64 = {
                let created_at = message_id.timestamp() as u128;
                ((now - created_at) / 1000) as i64
            };

            let min_age = config.resolved.older_than;
            let max_age = config.resolved.newer_than;

            let self_vote_valid = config.resolved.self_vote || reactor_id != message_author_id;

            let bots_valid = config.resolved.allow_bots || !message_author_is_bot;

            let images_valid = !config.resolved.require_image || (message_has_image == Some(true));

            let time_valid = {
                let min_age_valid = if min_age <= 0 {
                    true
                } else {
                    message_age > min_age
                };
                let max_age_valid = if max_age <= 0 {
                    true
                } else {
                    message_age < max_age
                };
                min_age_valid && max_age_valid
            };

            if self_vote_valid && bots_valid && images_valid && time_valid {
                Some((config, vote_type))
            } else {
                invalid_exists = true;
                None
            }
        };

        let mut upvote = Vec::new();
        let mut downvote = Vec::new();

        configs
            .into_iter()
            .filter_map(eval_config)
            .for_each(|(config, vote_type)| {
                if vote_type == VoteType::Upvote {
                    upvote.push(config)
                } else {
                    downvote.push(config)
                }
            });

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
