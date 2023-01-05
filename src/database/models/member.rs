#[derive(Debug)]
pub struct Member {
    pub user_id: i64,
    pub guild_id: i64,
    pub xp: f32,
    pub autoredeem_enabled: bool,
}

impl Member {
    pub async fn create(pool: &sqlx::PgPool, user_id: i64, guild_id: i64) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            "INSERT INTO members (user_id, guild_id) VALUES ($1, $2) RETURNING *",
            user_id,
            guild_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn set_xp(
        pool: &sqlx::PgPool,
        user_id: i64,
        guild_id: i64,
        xp: f32,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "UPDATE members SET xp=$1 WHERE user_id=$2 AND guild_id=$3 RETURNING *",
            xp,
            user_id,
            guild_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn list_by_xp(
        pool: &sqlx::PgPool,
        guild_id: i64,
        limit: i64,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM members WHERE guild_id=$1 AND xp > 0 ORDER BY xp DESC LIMIT $2",
            guild_id,
            limit,
        )
        .fetch_all(pool)
        .await
    }
}
