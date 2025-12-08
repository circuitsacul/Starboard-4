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

use snafu::ErrorCompat;
use tokio::main;
use tracing_subscriber::{EnvFilter, fmt};

use crate::client::{bot::StarboardBot, config::Config, runner::run};

fn init_tracing() {
    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .finish(),
    )
    .expect("Unable to set global tracing subscriber");
}

#[main]
async fn main() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    init_tracing();

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
            eprintln!("{}", &why);
            if let Some(bt) = ErrorCompat::backtrace(&why) {
                eprintln!("{:#?}", &bt);
            }
            sentry::capture_error(&why);
            return;
        }
    };

    run(bot).await;
}
