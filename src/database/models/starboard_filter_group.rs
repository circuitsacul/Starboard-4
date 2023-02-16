pub struct StarboardFilterGroup {
    pub filter_group_id: i32,
    pub starboard_id: i32,
}

impl StarboardFilterGroup {
    pub async fn list_by_starboard(
        pool: &sqlx::PgPool,
        starboard_id: i32,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM starboard_filter_groups WHERE starboard_id=$1",
            starboard_id
        )
        .fetch_all(pool)
        .await
    }
}
