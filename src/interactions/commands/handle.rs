use std::sync::Arc;

use twilight_interactions::command::CommandModel;
use twilight_model::application::interaction::ApplicationCommand;

use crate::{
    client::bot::StarboardBot,
    interactions::commands::{chat, context::CommandCtx},
};

macro_rules! match_commands {
    ($ctx:expr, $($cmd_name:expr => $command:ty),*) => {
        let data = $ctx.interaction.data.clone().into();
        let name = $ctx.interaction.data.name.as_str();
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
    interaction: Box<ApplicationCommand>,
) -> anyhow::Result<()> {
    let ctx = CommandCtx::new(shard_id, bot, interaction);

    match_commands!(
        ctx,
        "ping" => chat::ping::Ping,
        "autostar" => chat::autostar::AutoStar
    );

    Ok(())
}
