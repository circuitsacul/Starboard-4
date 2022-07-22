use crate::map_dup_none;

#[derive(Debug)]
pub struct Vote {
    pub message_id: i64,
    pub starboard_id: i32,
    pub user_id: i64,

    pub target_author_id: i64,
    pub is_downvote: bool,
}

impl Vote {
    pub async fn create(
        pool: &sqlx::PgPool,
        message_id: i64,
        starboard_id: i32,
        user_id: i64,
        target_author_id: i64,
        is_downvote: bool,
    ) -> sqlx::Result<Self> {
        let create = map_dup_none!(sqlx::query_as!(
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
        .fetch_one(pool))?;

        if let Some(create) = create {
            return Ok(create);
        }

        sqlx::query_as!(
            Self,
            r#"UPDATE votes SET is_downvote=$1 WHERE
            message_id=$2 AND starboard_id=$3 AND
            user_id=$4 RETURNING *"#,
            is_downvote,
            message_id,
            starboard_id,
            user_id,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        message_id: i64,
        starboard_id: i32,
        user_id: i64,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "DELETE FROM votes WHERE message_id=$1 AND starboard_id=$2 AND user_id=$3
            RETURNING *",
            message_id,
            starboard_id,
            user_id,
        )
        .fetch_optional(pool)
        .await
    }
}
