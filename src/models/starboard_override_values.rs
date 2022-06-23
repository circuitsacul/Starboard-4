use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum OverrideField<T> {
    Default,
    Value(T),
}

impl<T> Default for OverrideField<T> {
    fn default() -> Self {
        Self::Default
    }
}

impl<T> OverrideField<T> {
    fn is_default(&self) -> bool {
        match self {
            Self::Default => true,
            Self::Value(_) => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OverrideValues {
    // General Style
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub display_emoji: OverrideField<Option<String>>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub ping_author: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub use_server_profile: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub extra_embeds: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub use_webhook: OverrideField<bool>,

    // Embed Style
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub color: OverrideField<Option<i32>>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub jump_to_message: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub attachments_list: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub replied_to: OverrideField<bool>,

    // Requirements
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub required: OverrideField<i16>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub required_remove: OverrideField<i16>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub upvote_emojis: OverrideField<Vec<String>>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub downvote_emojis: OverrideField<Vec<String>>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub self_vote: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub allow_bots: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub require_image: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub older_than: OverrideField<i64>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub newer_than: OverrideField<i64>,

    // Behavior
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub enabled: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub autoreact_upvote: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub autoreact_downvote: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub remove_invalid_reactions: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub link_deletes: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub link_edits: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub xp_multiplier: OverrideField<f32>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub cooldown_enabled: OverrideField<bool>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub cooldown_count: OverrideField<i16>,
    #[serde(skip_serializing_if = "OverrideField::is_default", default)]
    pub cooldown_period: OverrideField<i16>,
}
