use crate::database::OverrideValues;

#[derive(Debug)]
pub struct StarboardOverride {
    // serial
    pub id: i32,
    pub guild_id: i64,
    pub name: String,

    pub starboard_id: i32,
    pub channel_ids: Vec<i64>,

    overrides: serde_json::Value,
}

impl StarboardOverride {
    pub async fn create(
        pool: &sqlx::PgPool,
        guild_id: i64,
        name: &String,
        starboard_id: i32,
    ) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            r#"INSERT INTO overrides
            (guild_id, name, starboard_id)
            VALUES ($1, $2, $3)
            RETURNING *"#,
            guild_id,
            name,
            starboard_id,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        guild_id: i64,
        name: &String,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "DELETE FROM overrides WHERE guild_id=$1 AND name=$2 RETURNING *",
            guild_id,
            name,
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn list_by_guild(pool: &sqlx::PgPool, guild_id: i64) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM overrides WHERE guild_id=$1", guild_id)
            .fetch_all(pool)
            .await
    }

    pub async fn list_by_starboard_and_channels(
        pool: &sqlx::PgPool,
        starboard_id: i32,
        channel_ids: &[i64],
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            r#"SELECT * FROM overrides
            WHERE starboard_id=$1 AND
            channel_ids && $2::bigint[]"#,
            starboard_id,
            channel_ids,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn list_by_starboard(
        pool: &sqlx::PgPool,
        starboard_id: i32,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM overrides WHERE starboard_id=$1",
            starboard_id,
        )
        .fetch_all(pool)
        .await
    }

    pub fn get_overrides(&self) -> serde_json::Result<OverrideValues> {
        serde_json::from_value(self.overrides.clone())
    }
}
