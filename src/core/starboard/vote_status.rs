//! tool for checking if a vote is valid

use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};

use crate::client::bot::StarboardBot;

use super::config::StarboardConfig;

pub enum VoteStatus<'a> {
    Ignore,
    Remove,
    Valid(&'a [StarboardConfig]),
}

impl VoteStatus<'_> {
    pub async fn get_vote_status<'a>(
        _bot: &StarboardBot,
        configs: &'a Vec<StarboardConfig>,
        _message_id: Id<MessageMarker>,
        _channel_id: Id<ChannelMarker>,
    ) -> VoteStatus<'a> {
        VoteStatus::Valid(&configs[..])
    }
}
