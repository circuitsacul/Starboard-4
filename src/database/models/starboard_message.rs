#[derive(Debug)]
pub struct StarboardMessage {
    pub message_id: i64,
    pub starboard_id: i32,
    pub starboard_message_id: Option<i64>,
    pub last_known_point_count: i16,
}

impl StarboardMessage {
    pub async fn create(
        pool: &sqlx::PgPool,
        message_id: i64,
        starboard_id: i32,
    ) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            r#"INSERT INTO starboard_messages
            (message_id, starboard_id)
            VALUES ($1, $2)
            RETURNING *"#,
            message_id,
            starboard_id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }

    pub async fn get(
        pool: &sqlx::PgPool,
        message_id: i64,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM starboard_messages WHERE
            starboard_message_id=$1",
            message_id,
        )
        .fetch_optional(pool)
        .await
    }
}
