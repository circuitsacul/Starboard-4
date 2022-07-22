use twilight_model::gateway::payload::incoming::{
    ReactionAdd, ReactionRemove, ReactionRemoveAll, ReactionRemoveEmoji,
};

use crate::client::bot::StarboardBot;

pub async fn handle_reaction_add(
    _bot: &StarboardBot,
    _event: Box<ReactionAdd>,
) -> anyhow::Result<()> {
    todo!()
}

pub async fn handle_reaction_remove(
    _bot: &StarboardBot,
    _event: Box<ReactionRemove>,
) -> anyhow::Result<()> {
    todo!()
}

pub async fn handle_reaction_remove_all(
    _bot: &StarboardBot,
    _event: ReactionRemoveAll,
) -> anyhow::Result<()> {
    todo!()
}

pub async fn handle_reaction_remove_emoji(
    _bot: &StarboardBot,
    _event: ReactionRemoveEmoji,
) -> anyhow::Result<()> {
    todo!()
}
