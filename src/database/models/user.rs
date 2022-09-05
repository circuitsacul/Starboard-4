#[derive(Debug)]
pub struct User {
    pub user_id: i64,
    pub is_bot: bool,
    pub credits: i32,
    pub donated_cents: i64,
    /// 0=none, 1=active, 2=declined, 3=former
    pub patreon_status: i16,
}

impl User {
    pub async fn create(pool: &sqlx::PgPool, user_id: i64, is_bot: bool) -> sqlx::Result<Self> {
        sqlx::query_as!(
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

    pub async fn get(pool: &sqlx::PgPool, user_id: i64) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM users WHERE user_id=$1", user_id,)
            .fetch_optional(pool)
            .await
    }
}
