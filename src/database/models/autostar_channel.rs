use sqlx::FromRow;

use crate::{
    constants,
    database::helpers::{
        query::build_update::build_update, settings::autostar::call_with_autostar_settings,
    },
};

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

    pub async fn update_settings(self, pool: &sqlx::PgPool) -> sqlx::Result<Option<Self>> {
        let mut builder =
            sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE autostar_channels SET ");

        call_with_autostar_settings!(build_update, self, builder);

        builder
            .push(" WHERE name=")
            .push_bind(self.name)
            .push(" AND guild_id=")
            .push_bind(self.guild_id)
            .push(" RETURNING *");

        let ret = builder.build().fetch_optional(pool).await?;

        if let Some(ret) = ret {
            Ok(Some(AutoStarChannel::from_row(&ret)?))
        } else {
            Ok(None)
        }
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

    pub async fn count_by_guild(pool: &sqlx::PgPool, guild_id: i64) -> sqlx::Result<i64> {
        sqlx::query!(
            "SELECT COUNT(*) as count FROM autostar_channels WHERE guild_id=$1",
            guild_id
        )
        .fetch_one(pool)
        .await
        .map(|r| r.count.unwrap())
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

    // validation
    pub fn set_emojis(&mut self, val: Vec<String>, premium: bool) -> Result<(), String> {
        let limit = if premium {
            constants::MAX_PREM_ASC_EMOJIS
        } else {
            constants::MAX_ASC_EMOJIS
        };

        if val.len() > limit {
            return Err(format!(
                "You can only have up to {} emojis per autostar channel. The premium limit is {}.",
                limit,
                constants::MAX_PREM_ASC_EMOJIS,
            ));
        }

        self.emojis = val;
        Ok(())
    }

    pub fn set_min_chars(&mut self, val: i16) -> Result<(), String> {
        if let Some(max_chars) = self.max_chars {
            if val > max_chars {
                return Err("`min-chars` cannot be greater than `max-chars`.".to_string());
            }
        }

        if val > constants::MAX_MIN_CHARS {
            Err(format!(
                "`min-chars` cannot be greater than {}.",
                constants::MAX_MIN_CHARS
            ))
        } else if val < 0 {
            Err("`min-chars` cannot be less than 0.".to_string())
        } else {
            self.min_chars = val;
            Ok(())
        }
    }

    pub fn set_max_chars(&mut self, val: Option<i16>) -> Result<(), String> {
        match val {
            None => {
                self.max_chars = None;
                Ok(())
            }
            Some(val) => {
                if val < self.min_chars {
                    Err("`max-chars` cannot be less than `min-chars.".to_string())
                } else if val < 0 {
                    Err("`max-chars` cannot be less than 0.".to_string())
                } else if val > constants::MAX_MAX_CHARS {
                    Err(format!(
                        "`max-chars` cannot be greater than {}.",
                        constants::MAX_MAX_CHARS
                    ))
                } else {
                    self.max_chars = Some(val);
                    Ok(())
                }
            }
        }
    }
}
