//! Utility for formatting starboard/override settings

use twilight_model::id::{marker::GuildMarker, Id};

use crate::{
    client::bot::StarboardBot,
    concat_format,
    core::{
        emoji::{EmojiCommon, SimpleEmoji},
        starboard::config::StarboardConfig,
    },
};

macro_rules! build_setting {
    ($ov_values: expr, $setting: ident, $pretty_name: expr, $value: expr) => {{
        let is_bold = match &$ov_values {
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

        format!("{}: {}", setting_name, $value)
    }};
}

macro_rules! join_settings {
    ($($setting: expr),*) => {{
        let mut final_result = String::new();
        $(
            final_result.push_str(&$setting);
        )*
        final_result
    }}
}

pub async fn format_settings(
    bot: &StarboardBot,
    guild_id: Id<GuildMarker>,
    config: &StarboardConfig,
) -> FormattedStarboardSettings {
    let ov_values = match config.overrides.get(0) {
        None => None,
        Some(ov) => Some(ov.get_overrides().unwrap()),
    };

    let display_emoji = SimpleEmoji::from_stored(
        config
            .resolved
            .display_emoji
            .clone()
            .unwrap_or(":star:".to_string()),
    )
    .into_readable(bot, guild_id)
    .await;

    FormattedStarboardSettings {
        style: join_settings!(build_setting!(
            ov_values,
            display_emoji,
            "display-emoji",
            display_emoji
        )),
        embed: concat_format!(
            "color:";
        ),
        requirements: join_settings!(build_setting!(
            ov_values,
            required,
            "required",
            config.resolved.required
        )),
        behavior: join_settings!(build_setting!(
            ov_values,
            autoreact_upvote,
            "autoreact-upvote",
            config.resolved.autoreact_upvote
        )),
    }
}

#[derive(Debug)]
pub struct FormattedStarboardSettings {
    pub style: String,
    pub embed: String,
    pub requirements: String,
    pub behavior: String,
}
