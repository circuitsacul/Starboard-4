#[derive(Debug)]
pub struct PosRoleMember {
    pub role_id: i64,
    pub user_id: i64,
}

impl PosRoleMember {
    pub async fn create(pool: &sqlx::PgPool, role_id: i64, user_id: i64) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            r#"INSERT INTO posrole_members
            (role_id, user_id)
            VALUES ($1, $2)
            RETURNING *"#,
            role_id,
            user_id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }
}
