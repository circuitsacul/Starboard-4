use std::sync::Arc;

use crate::{cache::models::message::CachedMessage, core::starboard::config::StarboardConfig};

pub struct Embedder<'config> {
    pub points: i32,
    pub config: &'config StarboardConfig,
    pub orig_message: Arc<Option<CachedMessage>>,
}

impl<'config> Embedder<'config> {
    pub fn new(
        points: i32,
        config: &'config StarboardConfig,
        orig_message: Arc<Option<CachedMessage>>,
    ) -> Self {
        Self {
            points,
            config,
            orig_message,
        }
    }
}
