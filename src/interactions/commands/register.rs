use std::sync::Arc;

use twilight_interactions::command::CreateCommand;

use crate::client::bot::Starboard;
use crate::interactions::commands::chat;

pub async fn post_commands(bot: Arc<Starboard>) {
    let inter_client = bot.interaction_client().await.unwrap();

    let mut commands = Vec::new();
    commands.push(chat::ping::Ping::create_command().into());

    match inter_client.set_global_commands(&commands).exec().await {
        Ok(_) => println!("Successfully registered commands"),
        Err(e) => eprintln!("Failed to register commands: {}", e),
    };
}
