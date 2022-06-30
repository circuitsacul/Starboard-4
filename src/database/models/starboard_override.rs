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
        id: i32,
        guild_id: i64,
        name: &String,
        starboard_id: i32,
    ) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            r#"INSERT INTO overrides
            (id, guild_id, name, starboard_id)
            VALUES ($1, $2, $3, $4)
            RETURNING *"#,
            id,
            guild_id,
            name,
            starboard_id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }

    pub fn get_overrides(&self) -> serde_json::Result<OverrideValues> {
        serde_json::from_value(self.overrides.clone()).map_err(|e| e.into())
    }
}
