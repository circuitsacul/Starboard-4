use anyhow::Result;
use sqlx::{query_as, PgPool};

pub struct PermRoleStarboard {
    pub permrole_id: i64,
    pub starboard_id: i32,

    pub give_votes: Option<bool>,
    pub receive_votes: Option<bool>,
}

impl PermRoleStarboard {
    pub async fn create(pool: &PgPool, permrole_id: i64, starboard_id: i32) -> Result<Self> {
        query_as!(
            Self,
            r#"INSERT INTO permrole_starboards
            (permrole_id, starboard_id)
            VALUES ($1, $2)
            RETURNING *"#,
            permrole_id,
            starboard_id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }
}
