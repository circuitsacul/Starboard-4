//! Utility for formatting starboard/override settings

use twilight_model::id::{marker::GuildMarker, Id};

use crate::{
    client::bot::StarboardBot,
    constants,
    core::{
        emoji::{EmojiCommon, SimpleEmoji},
        starboard::config::StarboardConfig,
    },
};

pub async fn format_settings(
    bot: &StarboardBot,
    guild_id: Id<GuildMarker>,
    config: &StarboardConfig,
) -> FormattedStarboardSettings {
    let ov_values = match config.overrides.get(0) {
        None => None,
        Some(ov) => Some(ov.get_overrides().unwrap()),
    };

    macro_rules! settings {
        ($($setting: ident, $pretty_name: expr, $value: expr;)*) => {{
            let mut final_result = String::new();
            $(
                let is_bold = match &ov_values {
                    None => false,
                    Some(ov) => match ov.$setting {
                        None => false,
                        Some(_) => true,
                    },
                };

                let setting_name = match is_bold {
                    true => format!("**{}**", $pretty_name),
                    false => $pretty_name.to_string(),
                };

                final_result.push_str(&format!("{}: {}\n", setting_name, $value));
            )*
            final_result
        }};
    }

    let res = &config.resolved;

    let display_emoji = SimpleEmoji::from_stored(
        config
            .resolved
            .display_emoji
            .clone()
            .unwrap_or(":star:".to_string()),
    )
    .into_readable(bot, guild_id)
    .await;

    let upvote_emojis = Vec::<SimpleEmoji>::from_stored(res.upvote_emojis.clone())
        .into_readable(bot, guild_id)
        .await;
    let downvote_emojis = Vec::<SimpleEmoji>::from_stored(res.downvote_emojis.clone())
        .into_readable(bot, guild_id)
        .await;

    let cooldown = {
        let is_bold = match &ov_values {
            None => false,
            Some(ov) => ov.cooldown_count.is_some() || ov.cooldown_period.is_some(),
        };

        let setting_name = match is_bold {
            false => "cooldown",
            true => "**cooldown**",
        };

        format!(
            "{}: {} reactions per {} seconds\n",
            setting_name, res.cooldown_count, res.cooldown_period
        )
    };

    let behavior = settings!(
        enabled, "enabled", res.enabled;
        autoreact_upvote, "autoreact-upvote", res.autoreact_upvote;
        autoreact_downvote, "autoreact-downvote", res.autoreact_downvote;
        remove_invalid_reactions, "remove-invalid-reactions", res.remove_invalid_reactions;
        link_deletes, "link-deletes", res.link_deletes;
        link_edits, "link-edits", res.link_edits;
        xp_multiplier, "xp-multiplier", res.xp_multiplier;
        cooldown_enabled, "cooldown-enabled", res.cooldown_enabled;
    ) + &cooldown
        + &format!("private: {}", res.private);

    FormattedStarboardSettings {
        style: settings!(
            display_emoji, "display-emoji", display_emoji;
            ping_author, "ping-author", res.ping_author;
            use_server_profile, "use-server-profile", res.use_server_profile;
            extra_embeds, "extra-embeds", res.extra_embeds;
            use_webhook, "use-webhook", res.use_webhook;
        ),
        embed: settings!(
            color, "color", &format!(
                "#{:X}", res.color.unwrap_or(constants::BOT_COLOR.try_into().unwrap())
            );
            jump_to_message, "jump-to-message", res.jump_to_message;
            attachments_list, "attachments-list", res.attachments_list;
            replied_to, "replied-to", res.replied_to;
        ),
        requirements: settings!(
            required, "required", res.required;
            required_remove, "required-remove", res.required_remove;
            upvote_emojis, "upvote-emojis", upvote_emojis;
            downvote_emojis, "downvote-emojis", downvote_emojis;
            self_vote, "self-vote", res.self_vote;
            allow_bots, "allow-bots", res.allow_bots;
            require_image, "require-image", res.require_image;
            older_than, "older-than", res.older_than;
            newer_than, "newer-than", res.newer_than;
        ),
        behavior,
    }
}

#[derive(Debug)]
pub struct FormattedStarboardSettings {
    pub style: String,
    pub embed: String,
    pub requirements: String,
    pub behavior: String,
}
