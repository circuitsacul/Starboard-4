use std::time::Duration;

use twilight_util::snowflake::Snowflake;

pub trait SnowflakeAge {
    /// Snowflake age in seconds
    fn age(&self) -> Duration;
}

impl<T: Snowflake> SnowflakeAge for T {
    fn age(&self) -> Duration {
        let now = chrono::Utc::now().timestamp_millis();

        Duration::from_micros((now - self.timestamp()) as u64)
    }
}
