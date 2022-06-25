use dotenv::dotenv;
use std::env;

pub struct Config {
    pub token: String,
    pub shards: u64,
    pub db_url: String,
    pub error_channel: Option<u64>,
}

impl Config {
    pub fn from_env() -> Self {
        match dotenv() {
            Ok(_) => {}
            Err(why) => eprintln!("Failed to load .env: {}", why),
        };
        let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
        let shards = env::var("SHARDS_PER_CLUSTER")
            .unwrap_or("1".to_string())
            .parse()
            .unwrap();
        let db_url = env::var("DATABASE_URL").expect("No database url specified.");
        let error_channel = env::var("ERROR_CHANNEL_ID")
            .ok()
            .map(|v| v.parse().expect("Invalid ID for error log channel."));

        Config {
            token,
            shards,
            db_url,
            error_channel,
        }
    }
}
