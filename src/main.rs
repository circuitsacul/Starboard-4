pub mod client;
pub mod events;
pub mod interactions;
pub mod models;

use anyhow::Result;

use crate::client::bot::StarboardBot;
use crate::client::config::Config;
use crate::client::runner::run;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env();
    let (events, starboard) = StarboardBot::new(config).await?;
    run(events, starboard).await;

    Ok(())
}
