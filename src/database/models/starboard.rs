use crate::database::{
    helpers::settings::starboard::call_with_starboard_settings,
    models::starboard_settings::generate_settings, StarboardSettings,
};

#[derive(Debug)]
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
            settings: call_with_starboard_settings!(generate_settings, $record),
        }
    };
}

impl Starboard {
    pub async fn create(
        pool: &sqlx::PgPool,
        name: &String,
        channel_id: i64,
        guild_id: i64,
    ) -> sqlx::Result<Self> {
        let starboard = sqlx::query!(
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

    pub async fn delete(
        pool: &sqlx::PgPool,
        name: &String,
        guild_id: i64,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query!(
            "DELETE FROM starboards WHERE name=$1 AND guild_id=$2
            RETURNING *",
            name,
            guild_id,
        )
        .fetch_optional(pool)
        .await
        .map(|row| row.map(|row| starboard_from_record!(row)))
    }

    pub async fn get_by_name(
        pool: &sqlx::PgPool,
        name: &str,
        guild_id: i64,
    ) -> sqlx::Result<Option<Self>> {
        let result = sqlx::query!(
            "SELECT * FROM starboards WHERE name=$1 AND guild_id=$2",
            name,
            guild_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|sb| starboard_from_record!(sb)))
    }

    pub async fn list_by_guild(pool: &sqlx::PgPool, guild_id: i64) -> sqlx::Result<Vec<Self>> {
        sqlx::query!("SELECT * FROM starboards WHERE guild_id=$1", guild_id,)
            .fetch_all(pool)
            .await
            .map(|rows| {
                rows.into_iter()
                    .map(|row| starboard_from_record!(row))
                    .collect()
            })
    }
}
