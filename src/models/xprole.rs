use anyhow::Result;
use sqlx::{query_as, PgPool};

pub struct XPRole {
    pub role_id: i64,
    pub guild_id: i64,
    pub required: i16,
}

impl XPRole {
    pub async fn create(pool: &PgPool, role_id: i64, guild_id: i64, required: i16) -> Result<Self> {
        query_as!(
            Self,
            r#"INSERT INTO xproles
            (role_id, guild_id, required)
            VALUES ($1, $2, $3)
            RETURNING *"#,
            role_id,
            guild_id,
            required,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }
}
