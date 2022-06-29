use sqlx::FromRow;

use super::helpers::query::build_update::build_update;
use super::helpers::settings::autostar::call_with_autostar_settings;

#[derive(Debug, FromRow)]
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
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "DELETE FROM autostar_channels
            WHERE name=$1 AND guild_id=$2
            RETURNING *",
            name,
            guild_id,
        )
        .fetch_optional(pool)
        .await
    }

    pub fn edit_settings() -> UpdateAutoStarChannel {
        UpdateAutoStarChannel::new()
    }

    pub async fn rename(
        pool: &sqlx::PgPool,
        name: &String,
        guild_id: i64,
        new_name: &String,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "UPDATE autostar_channels SET name=$1 WHERE name=$2 AND guild_id=$3
            RETURNING *",
            new_name,
            name,
            guild_id,
        )
        .fetch_optional(pool)
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

    pub async fn list_by_channel(pool: &sqlx::PgPool, channel_id: i64) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM autostar_channels WHERE channel_id = $1",
            channel_id,
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

#[derive(Default)]
pub struct UpdateAutoStarChannel {
    emojis: Option<Vec<String>>,
    min_chars: Option<i16>,
    max_chars: Option<Option<i16>>,
    require_image: Option<bool>,
    delete_invalid: Option<bool>,
}

impl UpdateAutoStarChannel {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_emojis(&mut self, emojis: Vec<String>) {
        self.emojis = Some(emojis);
    }

    pub fn set_min_chars(&mut self, value: i16) {
        self.min_chars = Some(value);
    }

    pub fn set_max_chars(&mut self, value: Option<i16>) {
        self.max_chars = Some(value);
    }

    pub fn set_require_image(&mut self, value: bool) {
        self.require_image = Some(value);
    }

    pub fn set_delete_invalid(&mut self, value: bool) {
        self.delete_invalid = Some(value);
    }

    pub async fn exec(
        self,
        pool: &sqlx::PgPool,
        name: &String,
        guild_id: i64,
    ) -> sqlx::Result<Option<AutoStarChannel>> {
        let mut builder =
            sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE autostar_channels SET ");

        call_with_autostar_settings!(build_update, self, builder);

        builder
            .push(" WHERE name=")
            .push_bind(name)
            .push(" AND guild_id=")
            .push_bind(guild_id)
            .push(" RETURNING *");

        builder
            .build()
            .fetch_optional(pool)
            .await
            .map(|r| r.map(|r| AutoStarChannel::from_row(&r).unwrap()))
    }
}
