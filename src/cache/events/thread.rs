use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::{
    ThreadCreate, ThreadDelete, ThreadListSync, ThreadUpdate,
};

use crate::cache::{cache::Cache, update::UpdateCache};

#[async_trait]
impl UpdateCache for ThreadCreate {
    async fn update_cache(&self, cache: &Cache) {
        println!("Thread created");
        let parent_id = match self.parent_id {
            None => return,
            Some(parent_id) => parent_id,
        };
        let guild_id = match self.guild_id {
            None => return,
            Some(guild_id) => guild_id,
        };

        cache.guilds.alter(&guild_id, |_, mut guild| {
            guild.active_thread_parents.insert(self.id, parent_id);
            guild
        })
    }
}

#[async_trait]
impl UpdateCache for ThreadDelete {
    async fn update_cache(&self, cache: &Cache) {
        println!("Thread delete.");
        cache.guilds.alter(&self.guild_id, |_, mut guild| {
            guild.active_thread_parents.remove(&self.id);
            guild
        })
    }
}

#[async_trait]
impl UpdateCache for ThreadUpdate {
    async fn update_cache(&self, cache: &Cache) {
        println!("Thread updated.");
        let guild_id = match self.guild_id {
            None => return,
            Some(guild_id) => guild_id,
        };
        let thread = match self.thread_metadata {
            None => return,
            Some(ref thread) => thread,
        };
        let parent_id = match self.parent_id {
            None => return,
            Some(parent_id) => parent_id,
        };

        cache.guilds.alter(&guild_id, |_, mut guild| {
            if thread.archived {
                guild.active_thread_parents.remove(&self.id);
            } else {
                guild.active_thread_parents.insert(self.id, parent_id);
            }

            guild
        })
    }
}

#[async_trait]
impl UpdateCache for ThreadListSync {
    async fn update_cache(&self, cache: &Cache) {
        println!("Thread list sync.");
        cache.guilds.alter(&self.guild_id, |_, mut guild| {
            if self.channel_ids.is_empty() {
                guild.active_thread_parents = self
                    .threads
                    .iter()
                    .map(|t| (t.id, t.parent_id.unwrap()))
                    .collect();
            } else {
                // ThreadListSync only syncs threads for the channels it sends,
                // so any threads belonging to other channels should stay.
                let channel_ids: HashSet<_> = self.channel_ids.iter().collect();
                let mut threads: HashMap<_, _> = guild
                    .active_thread_parents
                    .into_iter()
                    .filter(|(_, parent_id)| !channel_ids.contains(parent_id))
                    .collect();

                for thread in self.threads.iter() {
                    if let Some(parent_id) = thread.parent_id {
                        threads.insert(thread.id, parent_id);
                    }
                }

                guild.active_thread_parents = threads;
            }

            guild
        })
    }
}
