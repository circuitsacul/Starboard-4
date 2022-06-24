/// Unwrap a twilight ID marker intoan i64 value.
#[macro_export]
macro_rules! unwrap_id {
    ($id: expr) => {
        $id.get().try_into().unwrap()
    };
}
