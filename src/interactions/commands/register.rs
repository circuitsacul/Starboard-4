use std::vec::Vec;
use std::sync::Arc;

use twilight_model::application::command::Command;

use crate::client::bot::Starboard;
use crate::interactions::commands::chat;
use crate::interactions::commands::command::AppCommand;

fn build_register() -> Vec<Box<Command>> {
    (vec![chat::ping::PingCommand])
        .into_iter()
        .map(|command| Box::new(command.info()))
        .collect()
}

pub async fn post_commands(bot: Arc<Starboard>) {
    let commands = build_register();
}
