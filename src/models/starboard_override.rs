use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::{query_as, PgPool};

pub enum OverrideField<T> {
    Default,
    Override(T),
}

impl<T> Default for OverrideField<T> {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverrideValues {} // todo

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
        pool: &PgPool,
        id: i32,
        guild_id: i64,
        name: &String,
        starboard_id: i32,
    ) -> Result<Self> {
        query_as!(
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

    pub fn get_overrides(&self) -> Result<OverrideValues> {
        OverrideValues::deserialize(&self.overrides).map_err(|e| e.into())
    }
}
