use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OverrideValues {
    // General Style
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub display_emoji: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ping_author: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub use_server_profile: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub extra_embeds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub use_webhook: Option<bool>,

    // Embed Style
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub color: Option<Option<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub jump_to_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub attachments_list: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub replied_to: Option<bool>,

    // Requirements
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub required: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub required_remove: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub upvote_emojis: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub downvote_emojis: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub self_vote: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub allow_bots: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub require_image: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub older_than: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub newer_than: Option<i64>,

    // Behavior
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub autoreact_upvote: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub autoreact_downvote: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub remove_invalid_reactions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub link_deletes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub link_edits: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cooldown_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cooldown_count: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cooldown_period: Option<i16>,
}
