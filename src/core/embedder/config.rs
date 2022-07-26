use crate::core::starboard::config::StarboardConfig;

pub struct Embedder<'config> {
    pub points: i32,
    pub config: &'config StarboardConfig,
}

impl<'config> Embedder<'config> {
    pub fn new(points: i32, config: &'config StarboardConfig) -> Self {
        Self { points, config }
    }
}
