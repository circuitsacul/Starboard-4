use dotenv::dotenv;
use std::env;

pub struct Config {
    pub token: String,
    pub sentry: Option<String>,
    pub shards: u64,
    pub db_url: String,
    pub error_channel: Option<u64>,
    pub development: bool,
    pub owner_ids: Vec<u64>,
    pub bot_id: u64,
}

impl Config {
    pub fn from_env() -> Self {
        match dotenv() {
            Ok(_) => {}
            Err(why) => eprintln!("Failed to load .env: {why}"),
        };
        let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
        let sentry = env::var("SENTRY_URL").ok();
        let shards = env::var("SHARDS")
            .unwrap_or_else(|_| "1".to_string())
            .parse()
            .unwrap();
        let db_url = env::var("SB_DATABASE_URL").expect("No database url specified.");
        let error_channel = env::var("ERROR_CHANNEL_ID")
            .ok()
            .map(|v| v.parse().expect("Invalid ID for error log channel."));
        let development = env::var("DEVELOPMENT")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .expect("Invalid boolean for DEVELOPMENT.");
        let owner_ids = env::var("OWNER_IDS").ok().map(|var| {
            var.split(',')
                .map(|item| item.trim().parse().expect("invalid owner id"))
                .collect()
        });
        let bot_id = env::var("BOT_ID")
            .expect("No BOT_ID.")
            .parse()
            .expect("Invalid BOT_ID");

        Config {
            token,
            sentry,
            shards,
            db_url,
            error_channel,
            development,
            owner_ids: owner_ids.unwrap_or_default(),
            bot_id,
        }
    }
}
