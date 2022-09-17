use std::{
    collections::HashMap,
    hash::Hash,
    time::{Duration, Instant},
};

use tokio::sync::RwLock;

pub struct FixedMapping<K> {
    mapping: FlexibleMapping<K>,
    period: Duration,
    capacity: u32,
}

impl<K> FixedMapping<K>
where
    K: Hash + Eq + Clone,
{
    pub fn new(capacity: u32, period: Duration) -> Self {
        Self {
            mapping: FlexibleMapping::new(),
            period,
            capacity,
        }
    }

    pub async fn trigger(&self, key: K) -> Option<()> {
        self.mapping.trigger(key, self.capacity, self.period).await
    }
}

pub struct FlexibleMapping<K> {
    windows: RwLock<HashMap<K, JumpingWindow>>,
}

impl<K> FlexibleMapping<K>
where
    K: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            windows: RwLock::new(HashMap::new()),
        }
    }

    pub async fn trigger(&self, key: K, capacity: u32, period: Duration) -> Option<()> {
        let mut windows = self.windows.write().await;

        let mut to_remove = Vec::new();
        for window in windows.iter_mut() {
            if window.1.refresh() {
                to_remove.push(window.0.clone());
            }
        }
        for key in to_remove {
            windows.remove(&key);
        }

        let window = match windows.get_mut(&key) {
            None => {
                windows.insert(key.clone(), JumpingWindow::new(capacity, period));
                windows.get_mut(&key).unwrap()
            }
            Some(window) => window,
        };

        window.trigger()
    }
}

impl<K> Default for FlexibleMapping<K>
where
    K: Hash + Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct JumpingWindow {
    last_reset: Instant,
    period: Duration,
    capacity: u32,
    tokens: u32,
}

impl JumpingWindow {
    pub fn new(capacity: u32, period: Duration) -> Self {
        Self {
            last_reset: Instant::now(),
            period,
            capacity,
            tokens: capacity,
        }
    }

    pub fn trigger(&mut self) -> Option<()> {
        self.refresh();

        if self.tokens == 0 {
            return None;
        }

        self.tokens -= 1;

        Some(())
    }

    fn refresh(&mut self) -> bool {
        if self.last_reset.elapsed() >= self.period {
            self.tokens = self.capacity;
            self.last_reset = Instant::now();
            true
        } else {
            false
        }
    }
}
