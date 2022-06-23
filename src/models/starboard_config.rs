use anyhow::Result;

use crate::models::{OverrideField, Starboard, StarboardOverride, StarboardSettings};

pub struct StarboardConfig {
    pub starboard: Starboard,
    pub overrides: Vec<StarboardOverride>,
    pub resolved: StarboardSettings,
}

macro_rules! _update_from_override {
    ($settings: expr, $override: expr, $($field: ident),*) => {
        {
            $(
                match $override.$field {
                    OverrideField::Default => {},
                    OverrideField::Value(value) => $settings.$field = value,
                }
            )*
        }
    };
}

macro_rules! update_from_override {
    ($settings: expr, $override: expr) => {
        _update_from_override!(
            $settings,
            $override,
            display_emoji,
            ping_author,
            use_server_profile,
            extra_embeds,
            use_webhook,
            color,
            jump_to_message,
            attachments_list,
            replied_to,
            required,
            required_remove,
            upvote_emojis,
            downvote_emojis,
            self_vote,
            allow_bots,
            require_image,
            older_than,
            newer_than,
            enabled,
            autoreact_upvote,
            autoreact_downvote,
            remove_invalid_reactions,
            link_deletes,
            link_edits,
            xp_multiplier,
            cooldown_enabled,
            cooldown_count,
            cooldown_period
        )
    };
}

impl StarboardConfig {
    pub fn new(starboard: Starboard, overrides: Vec<StarboardOverride>) -> Result<Self> {
        let mut settings = starboard.settings.clone();
        for ov in overrides.iter() {
            update_from_override!(settings, ov.get_overrides().unwrap())
        }

        Ok(Self {
            starboard,
            overrides,
            resolved: settings,
        })
    }
}
