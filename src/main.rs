pub mod client;
pub mod events;
pub mod interactions;
pub mod utils;

use crate::client::bot::Starboard;
use crate::client::config::Config;
use crate::client::runner::run;
use crate::utils::types::Res;

#[tokio::main]
async fn main() -> Res {
    let config = Config::from_env();
    let (events, starboard) = Starboard::new(config).await?;
    run(events, starboard).await;

    Ok(())
}
