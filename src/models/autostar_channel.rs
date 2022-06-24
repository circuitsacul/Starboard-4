#[derive(Debug)]
pub struct AutoStarChannel {
    /// serial
    pub id: i32,
    pub name: String,
    pub channel_id: i64,
    pub guild_id: i64,

    pub premium_locked: bool,

    pub emojis: Vec<String>,
    pub min_chars: i16,
    pub max_chars: Option<i16>,
    pub require_image: bool,
    pub delete_invalid: bool,
}

impl AutoStarChannel {
    pub async fn create(
        pool: &sqlx::PgPool,
        name: &String,
        channel_id: i64,
        guild_id: i64,
    ) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            r#"INSERT INTO autostar_channels
                (name, channel_id, guild_id)
                VALUES ($1, $2, $3)
                RETURNING *"#,
            name,
            channel_id,
            guild_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        name: &String,
        guild_id: i64,
    ) -> sqlx::Result<sqlx::postgres::PgQueryResult> {
        sqlx::query!(
            "DELETE FROM autostar_channels
            WHERE name=$1 AND guild_id=$2",
            name,
            guild_id,
        )
        .execute(pool)
        .await
    }

    pub async fn list_by_guild(pool: &sqlx::PgPool, guild_id: i64) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM autostar_channels
            WHERE guild_id=$1",
            guild_id,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn get_by_name(
        pool: &sqlx::PgPool,
        name: &String,
        guild_id: i64,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM autostar_channels
            WHERE guild_id=$1 AND name=$2",
            guild_id,
            name,
        )
        .fetch_optional(pool)
        .await
    }
}
