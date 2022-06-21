use std::sync::Arc;

use twilight_interactions::command::CommandModel;
use twilight_model::application::interaction::ApplicationCommand;

use crate::client::bot::Starboard;
use crate::interactions::commands::chat;
use crate::interactions::commands::command::AppCommand;
use crate::interactions::commands::context::CommandCtx;

pub async fn handle_command(shard_id: u64, bot: Arc<Starboard>, command: Box<ApplicationCommand>) {
    let ctx = CommandCtx {
        shard_id,
        bot: Arc::clone(&bot),
        command,
    };

    let data = ctx.command.data.clone().into();
    match ctx.command.data.name.as_str() {
        "ping" => {
            chat::ping::Ping::from_interaction(data)
                .unwrap()
                .callback(ctx)
                .await
        }
        _ => {
            eprintln!("Unkown command: {}", ctx.command.data.name);
            return;
        }
    };
}
