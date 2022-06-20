mod commands;

use std::sync::Arc;

use twilight_model::application::interaction::Interaction;

use crate::interactions::commands::handle_command;
use crate::client::bot::Starboard;

pub async fn handle_interaction(shard_id: u64, interaction: Interaction, bot: Arc<Starboard>) {
    match interaction {
        Interaction::ApplicationCommand(command) => {
            handle_command(shard_id, Arc::clone(&bot), command).await;
        }
        _ => {}
    }
}
