//! Utility for formatting starboard/override settings

use std::{
    fmt::{Debug, Write},
    time::Duration,
};

use humantime::format_duration;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::{
    client::bot::StarboardBot,
    constants,
    core::{
        emoji::{EmojiCommon, SimpleEmoji},
        starboard::config::StarboardConfig,
    },
    database::ExclusiveGroup,
    errors::StarboardResult,
};

pub async fn format_settings(
    bot: &StarboardBot,
    guild_id: Id<GuildMarker>,
    config: &StarboardConfig,
) -> StarboardResult<FormattedStarboardSettings> {
    let ov_values = config
        .overrides
        .get(0)
        .map(|ov| ov.get_overrides().unwrap());

    macro_rules! settings {
        ($($setting: ident, $pretty_name: expr, $value: expr;)*) => {{
            let mut final_result = String::new();
            $(
                let is_bold = ov_values.as_ref().map_or(false, |ov| {
                    ov.$setting.is_some()
                });

                let setting_name = if is_bold {
                    format!("**{}**", $pretty_name)
                } else {
                    $pretty_name.to_string()
                };

                writeln!(final_result, "{}: {}", setting_name, $value).unwrap();
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
            .unwrap_or_else(|| "none".to_string()),
    )
    .into_readable(bot, guild_id);

    let upvote_emojis = Vec::from_stored(res.upvote_emojis.clone()).into_readable(bot, guild_id);
    let downvote_emojis =
        Vec::from_stored(res.downvote_emojis.clone()).into_readable(bot, guild_id);

    let older_than = if res.older_than <= 0 {
        "disabled".to_string()
    } else {
        format_duration(Duration::from_secs(res.older_than as u64)).to_string()
    };
    let newer_than = if res.newer_than <= 0 {
        "disabled".to_string()
    } else {
        format_duration(Duration::from_secs(res.newer_than as u64)).to_string()
    };

    let owner: String;
    let exclusive_group = {
        match res.exclusive_group {
            None => "*no group*",
            Some(group) => {
                let group = ExclusiveGroup::get(&bot.pool, group).await?;
                match group {
                    Some(group) => {
                        owner = group.name;
                        &owner
                    }
                    None => "*no group*",
                }
            }
        }
    };

    let cooldown = {
        let is_bold = ov_values.as_ref().map_or(false, |ov| {
            ov.cooldown_count.is_some() || ov.cooldown_period.is_some()
        });

        let setting_name = if is_bold { "**cooldown**" } else { "cooldown" };

        format!(
            "{}: {} reactions per {} seconds\n",
            setting_name, res.cooldown_count, res.cooldown_period
        )
    };
    let on_delete = match res.on_delete {
        0 => "Refresh",
        1 => "Ignore",
        2 => "Trash All",
        3 => "Freeze All",
        _ => "Invalid",
    };

    let behavior = settings!(
        enabled, "enabled", res.enabled;
        autoreact_upvote, "autoreact-upvote", res.autoreact_upvote;
        autoreact_downvote, "autoreact-downvote", res.autoreact_downvote;
        remove_invalid_reactions, "remove-invalid-reactions", res.remove_invalid_reactions;
        link_deletes, "link-deletes", res.link_deletes;
        link_edits, "link-edits", res.link_edits;
        on_delete, "on-delete", on_delete;
        cooldown_enabled, "cooldown-enabled", res.cooldown_enabled;
    ) + &cooldown
        + &format!("xp-multiplier: {}\n", res.xp_multiplier)
        + &format!("private: {}\n", res.private)
        + &settings!(
            exclusive_group, "exclusive-group", exclusive_group;
            exclusive_group_priority, "exclusive-group-priority", res.exclusive_group_priority;
        );

    let settings = FormattedStarboardSettings {
        style: settings!(
            display_emoji, "display-emoji", display_emoji;
            ping_author, "ping-author", res.ping_author;
            use_server_profile, "use-server-profile", res.use_server_profile;
            extra_embeds, "extra-embeds", res.extra_embeds;
            use_webhook, "use-webhook", res.use_webhook;
        ),
        embed: settings!(
            color, "color", &format!(
                "#{:X}", res.color.unwrap_or(constants::BOT_COLOR as i32)
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
            older_than, "older-than", older_than;
            newer_than, "newer-than", newer_than;
        ),
        behavior,
    };

    Ok(settings)
}

#[derive(Debug)]
pub struct FormattedStarboardSettings {
    pub style: String,
    pub embed: String,
    pub requirements: String,
    pub behavior: String,
}
