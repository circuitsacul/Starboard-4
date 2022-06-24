// Attempt to create & return a model, otherwise return None
#[macro_export]
macro_rules! create_maybe {
    ($model: ty, $($value: expr),*) => {{
        use sqlx::Error;
        match <$model>::create($($value,)*).await {
            Ok(value) => { Ok(Some(value)) },
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
