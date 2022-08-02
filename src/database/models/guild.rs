use chrono::{DateTime, Utc};

use crate::client::bot::StarboardBot;

#[derive(Debug)]
pub struct Guild {
    pub guild_id: i64,
    pub premium_end: Option<DateTime<Utc>>,
}

impl Guild {
    pub async fn create(pool: &sqlx::PgPool, guild_id: i64) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            "INSERT INTO guilds (guild_id) VALUES ($1) RETURNING *",
            guild_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }

    // helper methods
    pub async fn is_vote_emoji(
        bot: &StarboardBot,
        guild_id: i64,
        emoji_raw: &String,
    ) -> sqlx::Result<bool> {
        let is_cached_emoji = bot.cache.guild_vote_emojis.with(&guild_id, |_, emojis| {
            emojis.as_ref().map(|emojis| emojis.contains(emoji_raw))
        });

        if let Some(val) = is_cached_emoji {
            Ok(val)
        } else {
            let emojis = sqlx::query!(
                "SELECT ARRAY (
                    SELECT unnest(upvote_emojis || downvote_emojis)
                    FROM starboards
                    WHERE guild_id=$1
                )",
                guild_id,
            )
            .fetch_one(&bot.pool)
            .await?
            .array
            .unwrap_or_else(|| Vec::new());

            let is_vote = emojis.contains(emoji_raw);
            bot.cache.guild_vote_emojis.insert(guild_id, emojis);

            Ok(is_vote)
        }
    }
}
