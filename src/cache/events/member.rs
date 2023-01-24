use std::sync::Arc;

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::{MemberRemove, MemberUpdate};

use crate::cache::{cache_struct::Cache, update::UpdateCache};

#[async_trait]
impl UpdateCache for MemberRemove {
    async fn update_cache(&self, cache: &Cache) {
        cache.members.remove(&(self.guild_id, self.user.id)).await;
    }
}

#[async_trait]
impl UpdateCache for MemberUpdate {
    async fn update_cache(&self, cache: &Cache) {
        cache
            .members
            .insert_if_present(
                (self.guild_id, self.user.id),
                Some(Arc::new(self.into())),
                1,
            )
            .await;

        cache
            .users
            .insert_if_present(self.user.id, Some(Arc::new((&self.user).into())), 1)
            .await;
    }
}
