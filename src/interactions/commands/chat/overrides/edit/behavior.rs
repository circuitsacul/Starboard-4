use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation::cooldown::parse_cooldown, ExclusiveGroup, StarboardOverride},
    errors::StarboardResult,
    get_guild_id,
    interactions::{commands::choices::on_delete::OnDelete, context::CommandCtx},
    locale_func,
    utils::id_as_i64::GetI64,
};

locale_func!(overrides_edit_behavior);
locale_func!(overrides_edit_option_name);

locale_func!(sb_option_enabled);
locale_func!(sb_option_autoreact_upvote);
locale_func!(sb_option_autoreact_downvote);
locale_func!(sb_option_remove_invalid_reactions);
locale_func!(sb_option_link_deletes);
locale_func!(sb_option_link_edits);
locale_func!(sb_option_on_delete);
locale_func!(sb_option_cooldown_enabled);
locale_func!(sb_option_cooldown);
locale_func!(sb_option_exclusive_group);
locale_func!(sb_option_remove_exclusive_group);
locale_func!(sb_option_exclusive_group_priority);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "behavior",
    desc = "Edit how the starboard should behave.",
    desc_localizations = "overrides_edit_behavior"
)]
pub struct EditBehavior {
    /// The starboard to edit.
    #[command(autocomplete = true, desc_localizations = "overrides_edit_option_name")]
    name: String,

    /// Whether the starboard is enabled.
    #[command(desc_localizations = "sb_option_enabled")]
    enabled: Option<bool>,

    /// Whether to automatically react to starboard messages with the upvote emojis.
    #[command(
        rename = "autoreact-upvote",
        desc_localizations = "sb_option_autoreact_upvote"
    )]
    autoreact_upvote: Option<bool>,

    /// Whether to automatically react to starboard messages with the downvote emojis.
    #[command(
        rename = "autoreact-downvote",
        desc_localizations = "sb_option_autoreact_downvote"
    )]
    autoreact_downvote: Option<bool>,

    /// Whether to remove reactions that don't meet requirements.
    #[command(
        rename = "remove-invalid-reactions",
        desc_localizations = "sb_option_remove_invalid_reactions"
    )]
    remove_invalid_reactions: Option<bool>,

    /// If the original message is deleted, whether to also delete the starboard message.
    #[command(rename = "link-deletes", desc_localizations = "sb_option_link_deletes")]
    link_deletes: Option<bool>,

    /// If the original message is edted, whether to also update the content of the starboard message.
    #[command(rename = "link-edits", desc_localizations = "sb_option_link_edits")]
    link_edits: Option<bool>,

    /// What to do if a moderator removes a post from the starboard manually.
    #[command(rename = "on-delete", desc_localizations = "sb_option_on_delete")]
    on_delete: Option<OnDelete>,

    /// Whether to enable the per-user vote cooldown.
    #[command(
        rename = "cooldown-enabled",
        desc_localizations = "sb_option_cooldown_enabled"
    )]
    cooldown_enabled: Option<bool>,

    /// The size of the cooldown (e.x. "5/6" means 5 votes per 6 seconds).
    #[command(desc_localizations = "sb_option_cooldown")]
    cooldown: Option<String>,

    /// Add this starboard to an exclusive group (only one at a time).
    #[command(
        rename = "exclusive-group",
        autocomplete = true,
        desc_localizations = "sb_option_exclusive_group"
    )]
    exclusive_group: Option<String>,

    /// Remove this starboard from the exclusive group.
    #[command(
        rename = "remove-exclusive-group",
        desc_localizations = "sb_option_remove_exclusive_group"
    )]
    remove_exclusive_group: Option<bool>,

    #[command(
        rename = "exclusive-group-priority", min_value=-50, max_value=50,
        desc_localizations = "sb_option_exclusive_group_priority"
    )]
    /// Set the priority of this starboard in the exclusive group.
    exclusive_group_priority: Option<i64>,
}

impl EditBehavior {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();
        let lang = ctx.user_lang();

        let ov = match StarboardOverride::get(&ctx.bot.pool, guild_id, &self.name).await? {
            None => {
                ctx.respond_str(&lang.override_missing(self.name), true)
                    .await?;
                return Ok(());
            }
            Some(ov) => ov,
        };
        let mut settings = ov.get_overrides()?;

        if let Some(val) = self.enabled {
            settings.enabled = Some(val);
        }
        if let Some(val) = self.autoreact_upvote {
            settings.autoreact_upvote = Some(val);
        }
        if let Some(val) = self.autoreact_downvote {
            settings.autoreact_downvote = Some(val);
        }
        if let Some(val) = self.remove_invalid_reactions {
            settings.remove_invalid_reactions = Some(val);
        }
        if let Some(val) = self.link_deletes {
            settings.link_deletes = Some(val);
        }
        if let Some(val) = self.link_edits {
            settings.link_edits = Some(val);
        }
        if let Some(val) = self.on_delete {
            settings.on_delete = Some(val.value() as i16);
        }
        if let Some(val) = self.cooldown_enabled {
            settings.cooldown_enabled = Some(val);
        }
        if let Some(val) = self.cooldown {
            let (capacity, period) = match parse_cooldown(&val) {
                Err(why) => {
                    ctx.respond_str(&why, true).await?;
                    return Ok(());
                }
                Ok(val) => val,
            };
            settings.cooldown_count = Some(capacity);
            settings.cooldown_period = Some(period);
        }
        if let Some(val) = self.exclusive_group {
            let group = ExclusiveGroup::get_by_name(&ctx.bot.pool, guild_id, &val).await?;
            let Some(group) = group else {
                ctx.respond_str(&lang.exclusive_group_missing(val), true).await?;
                return Ok(());
            };
            settings.exclusive_group = Some(Some(group.id));
        }
        if let Some(val) = self.remove_exclusive_group {
            if val {
                settings.exclusive_group = Some(None);
            }
        }
        if let Some(val) = self.exclusive_group_priority {
            settings.exclusive_group_priority = Some(val as i16);
        }

        StarboardOverride::update_settings(&ctx.bot.pool, ov.id, settings).await?;
        ctx.respond_str(&lang.overrides_edit_done(self.name), false)
            .await?;

        Ok(())
    }
}
