use std::borrow::Cow;

pub fn get_pg_err_code(err: &sqlx::Error) -> Option<Cow<'_, str>> {
    match err {
        sqlx::Error::Database(err) => err.code(),
        _ => None,
    }
}
