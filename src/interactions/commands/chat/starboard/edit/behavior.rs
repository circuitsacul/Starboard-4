use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::channel::{ChannelFlags, ChannelType};

use crate::{
    database::{
        validation::{self, cooldown::parse_cooldown},
        ExclusiveGroup, Starboard,
    },
    errors::StarboardResult,
    get_guild_id,
    interactions::{commands::choices::on_delete::OnDelete, context::CommandCtx},
    utils::id_as_i64::GetI64,
};
use crate::utils::into_id::IntoId;

#[derive(CommandModel, CreateCommand)]
#[command(name = "behavior", desc = "Edit how the starboard should behave.")]
pub struct EditBehavior {
    /// The starboard to edit.
    #[command(autocomplete = true)]
    name: String,

    /// Whether the starboard is enabled.
    enabled: Option<bool>,
    /// Whether to automatically react to starboard messages with the upvote emojis.
    #[command(rename = "autoreact-upvote")]
    autoreact_upvote: Option<bool>,
    /// Whether to automatically react to starboard messages with the downvote emojis.
    #[command(rename = "autoreact-downvote")]
    autoreact_downvote: Option<bool>,
    /// Whether to remove reactions that don't meet requirements.
    #[command(rename = "remove-invalid-reactions")]
    remove_invalid_reactions: Option<bool>,
    /// If the original message is deleted, whether to also delete the starboard message.
    #[command(rename = "link-deletes")]
    link_deletes: Option<bool>,
    /// If the original message is edited, whether to also update the content of the starboard message.
    #[command(rename = "link-edits")]
    link_edits: Option<bool>,
    /// What to do if a moderator removes a post from the starboard manually.
    #[command(rename = "on-delete")]
    on_delete: Option<OnDelete>,
    /// If true, prevents /random and /moststarred from pulling from this starboard.
    private: Option<bool>,
    /// How much XP each upvote on this starboard counts for.
    #[command(rename = "xp-multiplier", min_value = -10, max_value = 10)]
    xp_multiplier: Option<f64>,
    /// Whether to enable the per-user vote cooldown.
    #[command(rename = "cooldown-enabled")]
    cooldown_enabled: Option<bool>,
    /// The size of the cooldown (e.x. "5/6" means 5 votes per 6 seconds).
    cooldown: Option<String>,
    /// Add this starboard to an exclusive group (only one at a time).
    #[command(rename = "exclusive-group", autocomplete = true)]
    exclusive_group: Option<String>,
    /// Remove this starboard from the exclusive group.
    #[command(rename = "remove-exclusive-group")]
    remove_exclusive_group: Option<bool>,
    #[command(rename = "exclusive-group-priority", min_value=-50, max_value=50)]
    /// Set the priority of this starboard in the exclusive group.
    exclusive_group_priority: Option<i64>,
    /// The tag to apply if the target channel is a forum
    #[command(rename = "forum-tag")]
    forum_tag: Option<String>,
    /// Stop applying a tag to posts if the target channel is a forum
    #[command(rename = "remove-forum-tag")]
    remove_forum_tag: Option<bool>,
}

impl EditBehavior {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();
        let mut starboard =
            match Starboard::get_by_name(&ctx.bot.pool, &self.name, guild_id).await? {
                None => {
                    ctx.respond_str("No starboard with that name was found.", true)
                        .await?;
                    return Ok(());
                }
                Some(starboard) => starboard,
            };

        if let Some(val) = self.enabled {
            starboard.settings.enabled = val;
        }
        if let Some(val) = self.autoreact_upvote {
            starboard.settings.autoreact_upvote = val;
        }
        if let Some(val) = self.autoreact_downvote {
            starboard.settings.autoreact_downvote = val;
        }
        if let Some(val) = self.remove_invalid_reactions {
            starboard.settings.remove_invalid_reactions = val;
        }
        if let Some(val) = self.link_deletes {
            starboard.settings.link_deletes = val;
        }
        if let Some(val) = self.link_edits {
            starboard.settings.link_edits = val;
        }
        if let Some(val) = self.on_delete {
            starboard.settings.on_delete = val.value() as i16;
        }
        if let Some(val) = self.private {
            starboard.settings.private = val;
        }
        if let Some(val) = self.xp_multiplier {
            let val = val.to_string().parse().unwrap();
            if let Err(why) = validation::starboard_settings::validate_xp_multiplier(val) {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            starboard.settings.xp_multiplier = val;
        }
        if let Some(val) = self.cooldown_enabled {
            starboard.settings.cooldown_enabled = val;
        }
        if let Some(val) = self.cooldown {
            let (capacity, period) = match parse_cooldown(&val) {
                Err(why) => {
                    ctx.respond_str(&why, true).await?;
                    return Ok(());
                }
                Ok(val) => val,
            };
            starboard.settings.cooldown_count = capacity;
            starboard.settings.cooldown_period = period;
        }
        if let Some(val) = self.exclusive_group {
            let group = ExclusiveGroup::get_by_name(&ctx.bot.pool, guild_id, &val).await?;
            let Some(group) = group else {
                ctx.respond_str(&format!(concat!(
                    "Exclusive group '{}' does not exist. If you meant to remove the exclusive ",
                    "group, use `remove-exclusive-group: True` instead."
                ), val), true).await?;
                return Ok(());
            };
            starboard.settings.exclusive_group = Some(group.id);
        }
        if let Some(val) = self.remove_exclusive_group {
            if val {
                starboard.settings.exclusive_group = None;
            }
        }
        if let Some(val) = self.exclusive_group_priority {
            starboard.settings.exclusive_group_priority = val as i16;
        }
        if let Some(forum_tag) = self.forum_tag {
            let channel = ctx.bot.http.channel(starboard.channel_id.into_id()).await?.model().await?;
            let channel_mention = channel.mention();

            if channel.kind == ChannelType::GuildForum && let Some(available_tags) = channel.available_tags {
                let tag = available_tags.into_iter().find(|available_tag| available_tag.name == forum_tag);
                if let Some(tag) = tag {
                    starboard.settings.forum_tag = Some(tag.id.get_i64());
                } else {
                    ctx.respond_str(
                        &format!("Tag '{}' not found in channel {}.", forum_tag, channel_mention),
                        true,
                    )
                        .await?;
                    return Ok(());
                }
            }
        }
        if let Some(val) = self.remove_forum_tag {
            if val {
                let channel = ctx.bot.http.channel(starboard.channel_id.into_id()).await?.model().await?;
                let channel_mention = channel.mention();

                let requires_tag = channel.flags.unwrap().contains(ChannelFlags::REQUIRE_TAG);
                if requires_tag {
                    ctx.respond_str(
                        &format!("Cannot remove forum-tag because channel '{}' requires a tag to post.", channel_mention),
                        true,
                    )
                        .await?;
                    return Ok(());
                }

                starboard.settings.forum_tag = None;
            }
        }

        starboard.update_settings(&ctx.bot.pool).await?;
        ctx.respond_str(
            &format!("Updated settings for starboard '{}'.", self.name),
            false,
        )
        .await?;

        Ok(())
    }
}
