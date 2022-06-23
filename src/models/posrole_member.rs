use anyhow::Result;
use sqlx::{PgPool, query_as};

pub struct PosRoleMember {
    pub role_id: i64,
    pub user_id: i64,
}

impl PosRoleMember {
    pub async fn create(pool: &PgPool, role_id: i64, user_id: i64) -> Result<Self> {
        query_as!(
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
