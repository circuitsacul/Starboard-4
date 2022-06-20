use std::sync::Arc;

use twilight_model::application::interaction::ApplicationCommand;

use crate::client::bot::Starboard;
use crate::interactions::commands::context::CommandCtx;

pub async fn handle_command(shard_id: u64, bot: Arc<Starboard>, command: Box<ApplicationCommand>) {
    let ctx = CommandCtx {
        shard_id,
        bot,
        command,
    };

    println!("Command! {:?}", ctx);
}
