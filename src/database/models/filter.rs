pub struct FilterGroup {
    pub id: i32,
    pub guild_id: i64,
    pub name: String,
}

impl FilterGroup {
    pub async fn create(
        pool: &sqlx::PgPool,
        guild_id: i64,
        name: &str,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "INSERT INTO filter_groups (guild_id, name) VALUES ($1, $2) ON CONFLICT DO NOTHING
            RETURNING *",
            guild_id,
            name
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        guild_id: i64,
        name: &str,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "DELETE FROM filter_groups WHERE guild_id=$1 AND name=$2 RETURNING *",
            guild_id,
            name
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn rename(pool: &sqlx::PgPool, id: i32, new_name: &str) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            "UPDATE filter_groups SET name=$1 WHERE id=$2 RETURNING *",
            new_name,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_name(
        pool: &sqlx::PgPool,
        guild_id: i64,
        name: &str,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM filter_groups WHERE guild_id=$1 AND name=$2",
            guild_id,
            name
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn list_by_guild(pool: &sqlx::PgPool, guild_id: i64) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM filter_groups WHERE guild_id=$1",
            guild_id
        )
        .fetch_all(pool)
        .await
    }
}

pub struct Filter {
    pub filter_group_id: i32,
    pub position: i16,

    pub instant_pass: bool,
    pub instant_fail: bool,

    // default context
    pub user_has_all_of: Option<Vec<i64>>,
    pub user_has_some_of: Option<Vec<i64>>,
    pub user_missing_all_of: Option<Vec<i64>>,
    pub user_missing_some_of: Option<Vec<i64>>,
    pub user_is_bot: Option<bool>,

    // message context
    pub in_channel: Option<Vec<i64>>,
    pub not_in_channel: Option<Vec<i64>>,
    pub in_channel_or_sub_channels: Option<Vec<i64>>,
    pub not_in_channel_or_sub_channels: Option<Vec<i64>>,
    pub min_attachments: Option<i16>,
    pub max_attachments: Option<i16>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub matches: Option<String>,
    pub not_matches: Option<String>,

    // vote context
    pub voter_has_all_of: Option<Vec<i64>>,
    pub voter_has_some_of: Option<Vec<i64>>,
    pub voter_missing_all_of: Option<Vec<i64>>,
    pub voter_missing_some_of: Option<Vec<i64>>,
    pub older_than: Option<i64>,
    pub newer_than: Option<i64>,
}

impl Filter {
    pub async fn create(
        pool: &sqlx::PgPool,
        filter_group_id: i32,
        position: i16,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "INSERT INTO filters (filter_group_id, position) VALUES ($1, $2)
            ON CONFLICT DO NOTHING RETURNING *",
            filter_group_id,
            position,
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        filter_group_id: i32,
        position: i16,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "DELETE FROM filters WHERE filter_group_id=$1 AND position=$2 RETURNING *",
            filter_group_id,
            position,
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get_last_position(pool: &sqlx::PgPool, filter_group_id: i32) -> sqlx::Result<i16> {
        sqlx::query!(
            "SELECT MAX(position) as position FROM filters WHERE filter_group_id=$1",
            filter_group_id
        )
        .fetch_one(pool)
        .await
        .map(|r| r.position.unwrap_or(0))
    }

    pub async fn shift(
        pool: &sqlx::PgPool,
        filter_group_id: i32,
        start: i16,
        end: Option<i16>,
        distance: i16,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE filters SET position = position + $1
            WHERE position >= $2 AND ($3::SMALLINT IS NULL OR position <= $3)
            AND filter_group_id=$4",
            distance,
            start,
            end,
            filter_group_id,
        )
        .execute(pool)
        .await
        .map(|_| ())
    }

    pub async fn list_by_filter(
        pool: &sqlx::PgPool,
        filter_group_id: i32,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM filters WHERE filter_group_id=$1 ORDER BY position DESC",
            filter_group_id
        )
        .fetch_all(pool)
        .await
    }
}
