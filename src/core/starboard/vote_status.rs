//! tool for checking if a vote is valid

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker, UserMarker},
    Id,
};
use twilight_util::snowflake::Snowflake;

use crate::{
    client::bot::StarboardBot,
    core::{emoji::SimpleEmoji, has_image::has_image, permroles::Permissions},
    errors::StarboardResult,
    utils::into_id::IntoId,
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
            None => bot
                .cache
                .fog_message(bot, channel_id, message_id)
                .await?
                .as_ref()
                .as_ref()
                .map(|msg| has_image(&msg.embeds, &msg.attachments)),
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
            let message_age = ((now as i128 - message_id.timestamp() as i128) / 1000) as i64;

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

        let mut invalid_exists_2 = false;

        for (config, vote_type) in configs.into_iter().filter_map(eval_config) {
            // check reactor/author role permissions
            let reactor_perms = Permissions::get_permissions(
                bot,
                reactor_id,
                config.starboard.guild_id.into_id(),
                Some(config.starboard.id),
            )
            .await?;
            let author_perms = Permissions::get_permissions(
                bot,
                message_author_id,
                config.starboard.guild_id.into_id(),
                Some(config.starboard.id),
            )
            .await?;

            if !reactor_perms.give_votes || !author_perms.receive_votes {
                invalid_exists_2 = true;
                continue;
            }

            // check cooldown
            if config.resolved.cooldown_enabled
                && bot
                    .cooldowns
                    .starboard_custom_cooldown
                    .trigger(
                        &(reactor_id, config.starboard.id),
                        config.resolved.cooldown_count as u64,
                        Duration::from_secs(config.resolved.cooldown_period as u64),
                    )
                    .is_some()
            {
                invalid_exists_2 = true;
                continue;
            }

            if vote_type == VoteType::Upvote {
                upvote.push(config);
            } else {
                downvote.push(config);
            }
        }

        if upvote.is_empty() && downvote.is_empty() {
            if (invalid_exists || invalid_exists_2) && allow_remove {
                Ok(VoteStatus::Remove)
            } else {
                Ok(VoteStatus::Ignore)
            }
        } else {
            Ok(VoteStatus::Valid((upvote, downvote)))
        }
    }
}
