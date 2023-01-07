#[derive(Debug)]
pub struct PosRole {
    pub role_id: i64,
    pub guild_id: i64,
    pub max_members: i32,
}

impl PosRole {
    pub async fn create(
        pool: &sqlx::PgPool,
        role_id: i64,
        guild_id: i64,
        max_members: i32,
    ) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            "INSERT INTO posroles (role_id, guild_id, max_members) VALUES ($1, $2, $3) RETURNING *",
            role_id,
            guild_id,
            max_members,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn set_max_members(
        pool: &sqlx::PgPool,
        role_id: i64,
        max_members: i32,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "UPDATE posroles SET max_members=$1 WHERE role_id=$2 RETURNING *",
            max_members,
            role_id,
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &sqlx::PgPool, role_id: i64) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "DELETE FROM posroles WHERE role_id=$1 RETURNING *",
            role_id,
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn list_by_guild(pool: &sqlx::PgPool, guild_id: i64) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM posroles WHERE guild_id=$1 ORDER BY max_members ASC",
            guild_id,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn count(pool: &sqlx::PgPool, guild_id: i64) -> sqlx::Result<i64> {
        Ok(
            sqlx::query!("SELECT count(*) FROM posroles WHERE guild_id=$1", guild_id)
                .fetch_one(pool)
                .await?
                .count
                .unwrap(),
        )
    }
}
