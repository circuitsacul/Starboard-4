use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct DbGuild {
    pub guild_id: i64,
    pub premium_end: Option<DateTime<Utc>>,
}

impl DbGuild {
    pub async fn create(pool: &sqlx::PgPool, guild_id: i64) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            "INSERT INTO guilds (guild_id) VALUES ($1) RETURNING *",
            guild_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get(pool: &sqlx::PgPool, guild_id: i64) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM guilds WHERE guild_id=$1", guild_id)
            .fetch_optional(pool)
            .await
    }
}
