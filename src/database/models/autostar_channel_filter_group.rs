pub struct AutostarChannelFilterGroup {
    pub filter_group_id: i32,
    pub autostar_channel_id: i32,
}

impl AutostarChannelFilterGroup {
    pub async fn create(
        pool: &sqlx::PgPool,
        filter_group_id: i32,
        autostar_channel_id: i32,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "INSERT INTO autostar_channel_filter_groups (filter_group_id, autostar_channel_id)
            VALUES ($1, $2) ON CONFLICT DO NOTHING RETURNING *",
            filter_group_id,
            autostar_channel_id,
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        filter_group_id: i32,
        autostar_channel_id: i32,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "DELETE FROM autostar_channel_filter_groups WHERE filter_group_id=$1 AND
            autostar_channel_id=$2 RETURNING *",
            filter_group_id,
            autostar_channel_id,
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn list_by_autostar_channel(
        pool: &sqlx::PgPool,
        autostar_channel_id: i32,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM autostar_channel_filter_groups WHERE autostar_channel_id=$1",
            autostar_channel_id
        )
        .fetch_all(pool)
        .await
    }
}
