pub struct StarboardSettings {
    // General Style
    pub display_emoji: Option<String>,
    pub ping_author: bool,
    pub use_server_profile: bool,
    pub extra_embeds: bool,
    pub use_webhook: bool,

    // Embed Style
    pub color: Option<i32>,
    pub jump_to_message: bool,
    pub attachments_list: bool,
    pub replied_to: bool,

    // Requirements
    pub required: i16,
    pub required_remove: i16,
    pub upvote_emojis: Vec<String>,
    pub downvote_emojis: Vec<String>,
    pub self_vote: bool,
    pub allow_bots: bool,
    pub require_image: bool,
    pub older_than: i64,
    pub newer_than: i64,

    // Behavior
    pub enabled: bool,
    pub autoreact_upvote: bool,
    pub autoreact_downvote: bool,
    pub remove_invalid_reactions: bool,
    pub link_deletes: bool,
    pub link_edits: bool,
    pub private: bool,
    pub xp_multiplier: f32,
    pub cooldown_enabled: bool,
    pub cooldown_count: i16,
    pub cooldown_period: i16,
}

#[macro_export]
macro_rules! _generate_settings {
    ($has_settings: expr, $($name: ident),*) => {
        StarboardSettings {
            $(
                $name: $has_settings.$name,
            )*
        }
    };
}

#[macro_export]
macro_rules! generate_settings {
    ($has_settings: expr) => {
        crate::_generate_settings!(
            $has_settings,
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
            cooldown_period,
            private
        )
    };
}
