#[derive(Debug)]
pub struct PosRole {
    pub role_id: i64,
    pub guild_id: i64,
    pub max_members: i32,
}

impl PosRole {
    pub async fn create(pool: &sqlx::PgPool, role_id: i64, guild_id: i64) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            r#"INSERT INTO posroles
            (role_id, guild_id)
            VALUES ($1, $2)
            RETURNING *"#,
            role_id,
            guild_id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }
}
