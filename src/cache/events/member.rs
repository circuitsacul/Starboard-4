use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::{MemberAdd, MemberChunk};

use crate::cache::{cache::Cache, update::UpdateCache};

#[async_trait]
impl UpdateCache for MemberChunk {
    async fn update_cache(&self, cache: &Cache) {
        for member in &self.members {
            cache.users.insert(member.user.id, (&member.user).into());
        }
    }
}

#[async_trait]
impl UpdateCache for MemberAdd {
    async fn update_cache(&self, cache: &Cache) {
        cache.users.insert(self.user.id, (&self.user).into());
    }
}
