use std::sync::Arc;

use twilight_model::application::interaction::ApplicationCommand;

use crate::client::bot::Starboard;
use crate::interactions::commands::context::CommandCtx;

pub async fn handle_command(shard_id: u64, bot: Arc<Starboard>, command: Box<ApplicationCommand>) {
    let command_callback = match bot.commands.get(&command.data.name) {
        Some(command) => command,
        None => {
            eprintln!("Command not found: {}", command.data.name);
            return;
        }
    };

    let ctx = CommandCtx {
        shard_id,
        bot: Arc::clone(&bot),
        command,
    };

    command_callback.callback(ctx).await;
}
