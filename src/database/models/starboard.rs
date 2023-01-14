use sqlx::Row;

use crate::database::{
    helpers::{
        query::build_update::build_update, settings::starboard::call_with_starboard_settings,
    },
    models::starboard_settings::{settings_from_record, settings_from_row},
    StarboardSettings,
};

#[derive(Debug, Clone)]
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
            settings: call_with_starboard_settings!(settings_from_record, $record),
        }
    };
}

macro_rules! starboard_from_row {
    ($record: expr) => {
        Starboard {
            id: $record.get("id"),
            name: $record.get("name"),
            channel_id: $record.get("channel_id"),
            guild_id: $record.get("guild_id"),
            webhook_id: $record.get("webhook_id"),
            premium_locked: $record.get("premium_locked"),
            settings: call_with_starboard_settings!(settings_from_row, $record),
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

    pub async fn update_settings(self, pool: &sqlx::PgPool) -> sqlx::Result<Option<Self>> {
        let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE starboards SET ");

        call_with_starboard_settings!(build_update, self.settings, builder);

        builder
            .push(" WHERE name=")
            .push_bind(self.name)
            .push(" AND guild_id=")
            .push_bind(self.guild_id)
            .push(" RETURNING *");

        builder
            .build()
            .fetch_optional(pool)
            .await
            .map(|r| r.map(|r| starboard_from_row!(r)))
    }

    pub async fn set_webhook(
        pool: &sqlx::PgPool,
        starboard_id: i32,
        webhook_id: Option<i64>,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE starboards SET webhook_id=$1 WHERE id=$2",
            webhook_id,
            starboard_id
        )
        .fetch_optional(pool)
        .await?;
        Ok(())
    }

    pub async fn disable_webhooks(pool: &sqlx::PgPool, starboard_id: i32) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE starboards SET use_webhook=false WHERE id=$1",
            starboard_id
        )
        .fetch_optional(pool)
        .await?;
        Ok(())
    }

    pub async fn rename(
        pool: &sqlx::PgPool,
        name: &String,
        guild_id: i64,
        new_name: &String,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query!(
            "UPDATE starboards SET name=$1 WHERE name=$2 AND guild_id=$3
            RETURNING *",
            new_name,
            name,
            guild_id,
        )
        .fetch_optional(pool)
        .await
        .map(|r| r.map(|r| starboard_from_record!(r)))
    }

    pub async fn get(pool: &sqlx::PgPool, id: i32) -> sqlx::Result<Option<Self>> {
        let result = sqlx::query!("SELECT * FROM starboards WHERE id=$1", id)
            .fetch_optional(pool)
            .await?;

        Ok(result.map(|sb| starboard_from_record!(sb)))
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

    pub async fn count_by_guild(pool: &sqlx::PgPool, guild_id: i64) -> sqlx::Result<i64> {
        sqlx::query!(
            "SELECT COUNT(*) as count FROM starboards WHERE guild_id=$1",
            guild_id
        )
        .fetch_one(pool)
        .await
        .map(|r| r.count.unwrap())
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
