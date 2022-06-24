use std::sync::Arc;

use twilight_interactions::command::CommandModel;
use twilight_model::application::interaction::ApplicationCommand;

use crate::client::bot::StarboardBot;
use crate::interactions::commands::chat;
use crate::interactions::commands::context::CommandCtx;

macro_rules! match_commands {
    ($ctx:expr, $($cmd_name:expr => $command:ty),*) => {
        let data = $ctx.command.data.clone().into();
        let name = $ctx.command.data.name.as_str();
        match name {
            $(
                $cmd_name => <$command>::from_interaction(data)?.callback($ctx).await?,
            )*
            unkown => eprintln!("Unkown command: {}", unkown),
        }
    };
}

pub async fn handle_command(
    shard_id: u64,
    bot: Arc<StarboardBot>,
    command: Box<ApplicationCommand>,
) -> anyhow::Result<()> {
    let ctx = CommandCtx {
        shard_id,
        bot: Arc::clone(&bot),
        command,
    };

    match_commands!(
        ctx,
        "ping" => chat::ping::Ping,
        "autostar" => chat::autostar::AutoStar
    );

    Ok(())
}
