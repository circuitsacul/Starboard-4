use starboard_rs::bot::Starboard;
use starboard_rs::config::Config;
use starboard_rs::types::Res;
use starboard_rs::runner::run;

#[tokio::main]
async fn main() -> Res {
    let config = Config::from_env();
    let starboard = Starboard::new(config).await?;
    run(starboard).await;

    Ok(())
}
