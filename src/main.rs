pub mod cache;
pub mod client;
pub mod constants;
pub mod core;
pub mod database;
pub mod errors;
pub mod events;
pub mod interactions;
pub mod macros;
pub mod owner;
pub mod utils;

use tokio::main;

use crate::client::{bot::StarboardBot, config::Config, runner::run};

#[main]
async fn main() {
    let config = Config::from_env();

    let _sentry_guard = config.sentry.as_ref().map(|url| {
        sentry::init((
            url.to_owned(),
            sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            },
        ))
    });

    let (events, starboard) = match StarboardBot::new(config).await {
        Ok(val) => val,
        Err(why) => {
            sentry::capture_error(&why);
            return;
        }
    };

    run(events, starboard).await;
}
