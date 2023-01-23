use futures::TryStreamExt;

use crate::{cache::Cache, utils::into_id::IntoId};

#[derive(Debug)]
pub struct DbMember {
    pub user_id: i64,
    pub guild_id: i64,
    pub xp: f32,
    pub autoredeem_enabled: bool,
}

impl DbMember {
    pub async fn create(
        pool: &sqlx::PgPool,
        user_id: i64,
        guild_id: i64,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "INSERT INTO members (user_id, guild_id) VALUES ($1, $2)
            ON CONFLICT DO NOTHING RETURNING *",
            user_id,
            guild_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn set_xp(
        pool: &sqlx::PgPool,
        user_id: i64,
        guild_id: i64,
        xp: f32,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "UPDATE members SET xp=$1 WHERE user_id=$2 AND guild_id=$3 RETURNING *",
            xp,
            user_id,
            guild_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn set_autoredeem_enabled(
        pool: &sqlx::PgPool,
        user_id: i64,
        guild_id: i64,
        autoredeem_enabled: bool,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE members SET autoredeem_enabled=$1 WHERE user_id=$2 AND guild_id=$3",
            autoredeem_enabled,
            user_id,
            guild_id,
        )
        .fetch_all(pool)
        .await?;

        Ok(())
    }

    pub async fn list_by_xp(
        pool: &sqlx::PgPool,
        guild_id: i64,
        limit: i64,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM members WHERE guild_id=$1 AND xp > 0 ORDER BY xp DESC LIMIT $2",
            guild_id,
            limit,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn list_by_xp_exclude_deleted(
        pool: &sqlx::PgPool,
        guild_id: i64,
        limit: i64,
        cache: &Cache,
    ) -> sqlx::Result<Vec<Self>> {
        let mut cursor = sqlx::query_as!(
            Self,
            "SELECT * FROM members WHERE guild_id=$1 AND xp > 0 ORDER BY xp DESC LIMIT $2",
            guild_id,
            limit,
        )
        .fetch(pool);

        let guild_id_id = guild_id.into_id();
        let filter = |user_id: i64| {
            cache.guilds.with(&guild_id_id, |_, guild| {
                guild
                    .as_ref()
                    .map(|guild| guild.members.contains_key(&user_id.into_id()))
                    .unwrap_or(false)
            })
        };

        let mut ret = Vec::new();
        while let Some(row) = cursor.try_next().await? {
            if !filter(row.user_id) {
                continue;
            }

            ret.push(row);
        }

        Ok(ret)
    }

    pub async fn list_autoredeem_by_user(
        pool: &sqlx::PgPool,
        user_id: i64,
    ) -> sqlx::Result<Vec<i64>> {
        let rows = sqlx::query!(
            "SELECT guild_id FROM members WHERE user_id=$1 AND autoredeem_enabled=true",
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.guild_id).collect())
    }

    pub async fn get(
        pool: &sqlx::PgPool,
        guild_id: i64,
        user_id: i64,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM members WHERE guild_id=$1 AND user_id=$2",
            guild_id,
            user_id,
        )
        .fetch_optional(pool)
        .await
    }
}
