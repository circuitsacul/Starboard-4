use anyhow::Result;
use sqlx::{query_as, PgPool};

pub struct Vote {
    pub message_id: i64,
    pub starboard_id: i32,
    pub user_id: i64,

    pub target_author_id: i64,
    pub is_downvote: bool,
}

impl Vote {
    pub async fn create(
        pool: &PgPool,
        message_id: i64,
        starboard_id: i32,
        user_id: i64,
        target_author_id: i64,
        is_downvote: bool,
    ) -> Result<Self> {
        query_as!(
            Self,
            r#"INSERT INTO VOTES
            (message_id, starboard_id, user_id,
                target_author_id, is_downvote)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *"#,
            message_id,
            starboard_id,
            user_id,
            target_author_id,
            is_downvote,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }
}
