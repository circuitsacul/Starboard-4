pub struct FilterGroup {
    pub id: i32,
    pub guild_id: i64,
    pub name: String,
}

impl FilterGroup {
    pub async fn create(
        pool: &sqlx::PgPool,
        guild_id: i64,
        name: &str,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "INSERT INTO filter_groups (guild_id, name) VALUES ($1, $2) ON CONFLICT DO NOTHING
            RETURNING *",
            guild_id,
            name
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        guild_id: i64,
        name: &str,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "DELETE FROM filter_groups WHERE guild_id=$1 AND name=$2 RETURNING *",
            guild_id,
            name
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn rename(pool: &sqlx::PgPool, id: i32, new_name: &str) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            "UPDATE filter_groups SET name=$1 WHERE id=$2 RETURNING *",
            new_name,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get(pool: &sqlx::PgPool, id: i32) -> sqlx::Result<Self> {
        sqlx::query_as!(Self, "SELECT * FROM filter_groups WHERE id=$1", id)
            .fetch_one(pool)
            .await
    }

    pub async fn get_by_name(
        pool: &sqlx::PgPool,
        guild_id: i64,
        name: &str,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM filter_groups WHERE guild_id=$1 AND name=$2",
            guild_id,
            name
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn list_by_guild(pool: &sqlx::PgPool, guild_id: i64) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM filter_groups WHERE guild_id=$1",
            guild_id
        )
        .fetch_all(pool)
        .await
    }
}
