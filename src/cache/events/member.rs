use std::sync::Arc;

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::{
    MemberAdd, MemberChunk, MemberRemove, MemberUpdate,
};

use crate::cache::{cache::Cache, update::UpdateCache};

#[async_trait]
impl UpdateCache for MemberChunk {
    async fn update_cache(&self, cache: &Cache) {
        for member in &self.members {
            cache
                .users
                .insert(member.user.id, Some(Arc::new((&member.user).into())));
        }

        cache.guilds.alter(&self.guild_id, |_, mut g| {
            for member in &self.members {
                g.members.insert(member.user.id, Arc::new(member.into()));
            }
            g
        })
    }
}

#[async_trait]
impl UpdateCache for MemberAdd {
    async fn update_cache(&self, cache: &Cache) {
        cache
            .users
            .insert(self.user.id, Some(Arc::new((&self.user).into())));

        cache.guilds.alter(&self.guild_id, |_, mut g| {
            g.members.insert(self.user.id, Arc::new((&self.0).into()));
            g
        });
    }
}

#[async_trait]
impl UpdateCache for MemberRemove {
    async fn update_cache(&self, cache: &Cache) {
        cache.guilds.alter(&self.guild_id, |_, mut g| {
            g.members.remove(&self.user.id);
            g
        });
    }
}

#[async_trait]
impl UpdateCache for MemberUpdate {
    async fn update_cache(&self, cache: &Cache) {
        cache.guilds.alter(&self.guild_id, |_, mut g| {
            g.members.insert(self.user.id, Arc::new(self.into()));
            g
        })
    }
}
