pub struct AutostarChannelFilterGroup {
    pub filter_group_id: i32,
    pub autostar_channel_id: i32,
}

impl AutostarChannelFilterGroup {
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
