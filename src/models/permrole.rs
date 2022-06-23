use anyhow::Result;
use sqlx::{PgPool, query_as};

pub struct PermRole {
    pub role_id: i64,
    pub guild_id: i64,

    pub obtain_xproles: Option<bool>,
    pub give_votes: Option<bool>,
    pub receive_votes: Option<bool>,
}

impl PermRole {
    pub async fn create(pool: &PgPool, role_id: i64, guild_id: i64) -> Result<Self> {
        query_as!(
            Self,
            r#"INSERT INTO permroles
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
