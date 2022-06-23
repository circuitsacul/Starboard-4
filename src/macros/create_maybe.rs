// Attempt to create & return a model, otherwise return None
#[macro_export]
macro_rules! create_maybe {
    ($model: ty, $($value: expr),*) => {
        match <$model>::create($($value,)*).await {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    };
}
