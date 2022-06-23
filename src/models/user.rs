use anyhow::Result;
use sqlx::{query_as, PgPool};

pub struct User {
    pub user_id: i64,
    pub is_bot: bool,
    pub credits: i32,
    pub donated_cents: i64,
    /// 0=none, 1=active, 2=declined, 3=former
    pub patreon_status: i16,
}

impl User {
    pub async fn create(pool: &PgPool, user_id: i64, is_bot: bool) -> Result<Self> {
        query_as!(
            Self,
            r#"INSERT INTO users
            (user_id, is_bot)
            VALUES ($1, $2)
            RETURNING *"#,
            user_id,
            is_bot,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }
}
