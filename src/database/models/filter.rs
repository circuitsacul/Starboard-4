pub struct FilterGroup {
    pub id: i32,
    pub guild_id: i64,
    pub name: String,
}

impl FilterGroup {
    pub async fn get_many(pool: &sqlx::PgPool, id: &[i32]) -> sqlx::Result<Self> {
        sqlx::query_as!(Self, "SELECT * FROM filter_groups WHERE id=any($1)", id)
            .fetch_one(pool)
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
    pub async fn list_by_filter(pool: &sqlx::PgPool, filter_group_id: i32) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM filters WHERE filter_group_id=$1 ORDER BY position DESC",
            filter_group_id
        )
        .fetch_all(pool)
        .await
    }
}
