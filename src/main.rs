pub mod utils;
pub mod events;
pub mod client;

use crate::client::bot::Starboard;
use crate::client::config::Config;
use crate::client::runner::run;
use crate::utils::types::Res;

#[tokio::main]
async fn main() -> Res {
    let config = Config::from_env();
    let starboard = Starboard::new(config).await?;
    run(starboard).await;

    Ok(())
}
