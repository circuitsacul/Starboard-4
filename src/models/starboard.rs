use anyhow::Result;
use sqlx::{query, PgPool};

use crate::models::StarboardSettings;

pub struct Starboard {
    pub id: i32,
    pub name: String,
    pub channel_id: i64,
    pub guild_id: i64,

    pub webhook_id: Option<i64>,
    pub premium_locked: bool,

    pub settings: StarboardSettings,
}

macro_rules! starboard_from_record {
    ($record: expr) => {
        Starboard {
            id: $record.id,
            name: $record.name,
            channel_id: $record.channel_id,
            guild_id: $record.guild_id,
            webhook_id: $record.webhook_id,
            premium_locked: $record.premium_locked,
            settings: generate_settings!($record),
        }
    };
}

impl Starboard {
    pub async fn create(
        pool: &PgPool,
        name: &String,
        channel_id: i64,
        guild_id: i64,
    ) -> Result<Self> {
        let starboard = query!(
            r#"INSERT INTO STARBOARDS
            (name, channel_id, guild_id)
            VALUES ($1, $2, $3)
            RETURNING *"#,
            name,
            channel_id,
            guild_id,
        )
        .fetch_one(pool)
        .await?;

        Ok(starboard_from_record!(starboard))
    }
}
