use std::sync::Arc;

use twilight_model::application::interaction::ApplicationCommand;

use crate::client::bot::Starboard;

#[derive(Debug)]
pub struct CommandCtx {
    pub shard_id: u64,
    pub bot: Arc<Starboard>,
    pub command: Box<ApplicationCommand>,
}
