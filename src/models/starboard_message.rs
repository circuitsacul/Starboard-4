use anyhow::Result;
use sqlx::{PgPool, query_as};

pub struct StarboardMessage {
    pub message_id: i64,
    pub starboard_id: i32,
    pub starboard_message_id: Option<i64>,
    pub last_known_point_count: i16,
}

impl StarboardMessage {
    pub async fn create(pool: &PgPool, message_id: i64, starboard_id: i32) -> Result<Self> {
        query_as!(
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
}
