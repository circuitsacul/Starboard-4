#[derive(Debug)]
pub struct Patron {
    pub patreon_id: String,
    pub discord_id: Option<i64>,
    pub last_patreon_total_cents: i64,
}

impl Patron {
    pub async fn create(pool: &sqlx::PgPool, patreon_id: String) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            r#"INSERT INTO patrons (patreon_id) VALUES ($1)
            RETURNING *"#,
            patreon_id,
        )
        .fetch_one(pool)
        .await
    }
}
