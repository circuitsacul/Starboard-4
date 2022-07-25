use crate::database::StarboardMessage;

#[derive(Debug)]
pub struct Message {
    pub message_id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub author_id: i64,

    pub is_nsfw: bool,

    pub forced_to: Vec<i32>,
    pub trashed: bool,
    pub trash_reason: Option<String>,
    pub frozen: bool,
}

impl Message {
    pub async fn create(
        pool: &sqlx::PgPool,
        message_id: i64,
        guild_id: i64,
        channel_id: i64,
        author_id: i64,
        is_nsfw: bool,
    ) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            r#"INSERT INTO messages (message_id, guild_id, channel_id,
                author_id, is_nsfw)
            VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
            message_id,
            guild_id,
            channel_id,
            author_id,
            is_nsfw,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }

    pub async fn get_original(pool: &sqlx::PgPool, message_id: i64) -> sqlx::Result<Option<Self>> {
        if let Some(sb_msg) = StarboardMessage::get(pool, message_id).await? {
            sqlx::query_as!(
                Self,
                "SELECT * FROM messages WHERE message_id=$1",
                sb_msg.message_id,
            )
            .fetch_optional(pool)
            .await
        } else {
            sqlx::query_as!(
                Self,
                "SELECT * FROM messages WHERE message_id=$1",
                message_id,
            )
            .fetch_optional(pool)
            .await
        }
    }
}
