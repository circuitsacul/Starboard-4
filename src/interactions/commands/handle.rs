use twilight_interactions::command::CommandModel;

use crate::interactions::{commands::chat, context::CommandCtx};

macro_rules! match_commands {
    ($ctx:expr, $($cmd_name:expr => $command:ty),*) => {
        let cmd_inp_data = (*$ctx.data).clone().into();
        match &*$ctx.data.name {
            $(
                $cmd_name => <$command>::from_interaction(cmd_inp_data)?.callback($ctx).await?,
            )*
            unknown => eprintln!("Unknown command: {}", unknown),
        }
    };
}

pub async fn handle_command(ctx: CommandCtx) -> anyhow::Result<()> {
    match_commands!(
        ctx,
        "ping" => chat::ping::Ping,
        "autostar" => chat::autostar::AutoStar,
        "starboards" => chat::starboard::Starboard,
        "overrides" => chat::overrides::Overrides
    );

    Ok(())
}
