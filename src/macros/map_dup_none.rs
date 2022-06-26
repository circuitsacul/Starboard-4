/// Maps a duplicate error into a None value.
#[macro_export]
macro_rules! map_dup_none {
    ($future: expr) => {{
        use sqlx::Error;
        match $future.await {
            Ok(value) => Ok(Some(value)),
            Err(Error::Database(db_err)) => match db_err.code() {
                Some(code) => match &*code {
                    "23505" => Ok(None),
                    _ => Err(Error::Database(db_err)),
                },
                _ => Err(Error::Database(db_err)),
            },
            Err(other) => Err(other),
        }
    }};
}
