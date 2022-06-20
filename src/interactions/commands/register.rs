use std::collections::HashMap;
use std::sync::Arc;

use crate::client::bot::Starboard;
use crate::interactions::commands::chat;
use crate::interactions::commands::command::AppCommand;

pub fn build_register() -> HashMap<String, Box<dyn AppCommand>> {
    let mut d: HashMap<String, Box<dyn AppCommand>> = HashMap::new();

    fn add_command(commands: &mut HashMap<String, Box<dyn AppCommand>>, command: impl AppCommand) {
        commands.insert(command.describe().name, Box::new(command));
    }

    add_command(&mut d, chat::ping::PingCommand);

    d
}

pub async fn post_commands(bot: Arc<Starboard>) {
    let inter_client = bot.interaction_client().await.unwrap();

    let mut commands = Vec::new();
    bot.commands.iter().for_each(|(_name, cmd)| {
        commands.push(cmd.describe());
    });

    match inter_client.set_global_commands(&commands).exec().await {
        Ok(_) => println!("Successfully registered commands"),
        Err(e) => eprintln!("Failed to register commands: {}", e),
    };
}
