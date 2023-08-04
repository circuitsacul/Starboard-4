pub mod cache;
pub mod client;
pub mod core;
pub mod events;
pub mod interactions;
pub mod macros;
pub mod owner;
pub mod parsing;
pub mod utils;

use tokio::main;

use common::config::Config;

use crate::client::{bot::StarboardBot, runner::run};

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

    let bot = match StarboardBot::new(config).await {
        Ok(val) => val,
        Err(why) => {
            eprintln!("{:#?}", &why);
            sentry::capture_error(&why);
            return;
        }
    };

    run(bot).await;
}
