use anyhow::Result;
use sqlx::{query_as, PgPool};

pub struct Member {
    pub user_id: i64,
    pub guild_id: i64,
    pub xp: f32,
    pub autoredeem_enabled: bool,
}

impl Member {
    pub async fn create(pool: &PgPool, user_id: i64, guild_id: i64) -> Result<Self> {
        query_as!(
            Self,
            "INSERT INTO members (user_id, guild_id) VALUES ($1, $2) RETURNING *",
            user_id,
            guild_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }
}
