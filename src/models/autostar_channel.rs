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
            r#"INSERT INTO aschannels
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
}
