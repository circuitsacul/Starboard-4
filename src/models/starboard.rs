use anyhow::Result;
use sqlx::{query_as, PgPool};

pub struct Starboard {
    pub id: i32,
    pub name: String,
    pub channel_id: i64,
    pub guild_id: i64,

    pub webhook_id: Option<i64>,
    pub premium_locked: bool,

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

impl Starboard {
    pub async fn create(
        pool: &PgPool,
        name: &String,
        channel_id: i64,
        guild_id: i64,
    ) -> Result<Self> {
        query_as!(
            Self,
            r#"INSERT INTO STARBOARDS
            (name, channel_id, guild_id)
            VALUES ($1, $2, $3)
            RETURNING *"#,
            name,
            channel_id,
            guild_id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }
}
