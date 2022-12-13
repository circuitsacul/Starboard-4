use std::time::{SystemTime, UNIX_EPOCH};

use twilight_util::snowflake::Snowflake;

pub trait IdAge {
    /// Snowflake age in seconds
    fn age(&self) -> i64;
}

impl<T: Snowflake> IdAge for T {
    fn age(&self) -> i64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        ((now as i128 - self.timestamp() as i128) / 1000) as i64
    }
}
