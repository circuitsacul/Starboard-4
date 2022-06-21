use dotenv::dotenv;
use std::env;

pub struct Config {
    pub token: String,
    pub shards_per_cluster: u64,
    pub total_clusters: u64,
}

impl Config {
    pub fn from_env() -> Self {
        match dotenv() {
            Ok(_) => {}
            Err(why) => eprintln!("Failed to load .env: {}", why)
        };
        let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
        let shards_per_cluster = env::var("SHARDS_PER_CLUSTER").unwrap_or("1".to_string());
        let total_clusters = env::var("TOTAL_CLUSTERS").unwrap_or("1".to_string());

        Config {
            token,
            shards_per_cluster: shards_per_cluster.parse().unwrap(),
            total_clusters: total_clusters.parse().unwrap(),
        }
    }
}
