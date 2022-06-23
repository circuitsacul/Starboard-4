use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{query_as, PgPool};

pub struct Guild {
    pub guild_id: i64,
    pub premium_end: Option<DateTime<Utc>>,
}

impl Guild {
    pub async fn create(pool: &PgPool, guild_id: i64) -> Result<Self> {
        query_as!(
            Self,
            "INSERT INTO guilds (guild_id) VALUES ($1) RETURNING *",
            guild_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }
}
