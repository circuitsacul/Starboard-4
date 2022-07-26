#[derive(Debug)]
pub struct StarboardMessage {
    pub message_id: i64,
    pub starboard_id: i32,
    pub starboard_message_id: i64,
    pub last_known_point_count: i16,
}

impl StarboardMessage {
    pub async fn create(
        pool: &sqlx::PgPool,
        message_id: i64,
        starboard_message_id: i64,
        starboard_id: i32,
        last_known_point_count: i32,
    ) -> sqlx::Result<Self> {
        let point_count: i16 = last_known_point_count.try_into().unwrap();
        sqlx::query_as!(
            Self,
            r#"INSERT INTO starboard_messages
            (message_id, starboard_id, starboard_message_id, last_known_point_count)
            VALUES ($1, $2, $3, $4)
            RETURNING *"#,
            message_id,
            starboard_id,
            starboard_message_id,
            point_count,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }

    pub async fn delete(
        pool: &sqlx::PgPool,
        starboard_message_id: i64,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "DELETE FROM starboard_messages WHERE starboard_message_id=$1
            RETURNING *",
            starboard_message_id,
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get(pool: &sqlx::PgPool, starboard_message_id: i64) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM starboard_messages WHERE
            starboard_message_id=$1",
            starboard_message_id,
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get_by_starboard(
        pool: &sqlx::PgPool,
        message_id: i64,
        starboard_id: i32,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM starboard_messages WHERE starboard_id=$1 AND message_id=$2",
            starboard_id,
            message_id,
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn set_last_point_count(
        pool: &sqlx::PgPool,
        starboard_message_id: i64,
        point_count: i16,
    ) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Self,
            "UPDATE starboard_messages SET last_known_point_count=$1 WHERE starboard_message_id=$2
            RETURNING *",
            point_count,
            starboard_message_id,
        )
        .fetch_optional(pool)
        .await
    }
}
