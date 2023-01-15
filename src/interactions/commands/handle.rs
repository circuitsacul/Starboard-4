use twilight_interactions::command::CommandModel;

use crate::{
    errors::StarboardResult,
    interactions::{commands::chat, context::CommandCtx},
};

macro_rules! match_commands {
    ($ctx:expr, $($cmd_name:expr => $command:ty),* $(,)?) => {
        let cmd_inp_data = $ctx.data.clone().into();
        match &*$ctx.data.name {
            $(
                $cmd_name => <$command>::from_interaction(cmd_inp_data)?.callback($ctx).await?,
            )*
            unknown => eprintln!("Unknown command: {}", unknown),
        }
    };
}

pub async fn handle_command(ctx: CommandCtx) -> StarboardResult<()> {
    match_commands!(
        ctx,
        "ping" => chat::ping::Ping,
        "help" => chat::help::Help,
        "leaderboard" => chat::leaderboard::Leaderboard,
        "stats" => chat::stats::Stats,
        "random" => chat::random::RandomPost,
        "moststarred" => chat::moststarred::Moststarred,
        "autostar" => chat::autostar::AutoStar,
        "starboards" => chat::starboard::Starboard,
        "overrides" => chat::overrides::Overrides,
        "permroles" => chat::permroles::PermRoles,
        "xproles" => chat::xproles::XPRoles,
        "posroles" => chat::posroles::PosRoles,
        "utils" => chat::utils::Utils,
        "premium" => chat::premium::Premium,
    );

    Ok(())
}
