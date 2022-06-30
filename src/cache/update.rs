use async_trait::async_trait;

use super::cache::Cache;

#[async_trait]
pub(super) trait UpdateCache {
    async fn update_cache(&self, cache: &Cache);
}
