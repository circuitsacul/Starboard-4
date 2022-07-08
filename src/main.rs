pub mod cache;
pub mod client;
pub mod constants;
pub mod core;
pub mod database;
pub mod events;
pub mod interactions;
pub mod macros;
pub mod owner;
pub mod utils;

use tokio::main;

use crate::client::{bot::StarboardBot, config::Config, runner::run};

#[main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env();
    let (events, starboard) = StarboardBot::new(config).await?;
    run(events, starboard).await;

    Ok(())
}
