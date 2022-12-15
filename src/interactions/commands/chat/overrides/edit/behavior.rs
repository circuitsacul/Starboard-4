use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{
        validation::{self, cooldown::parse_cooldown},
        StarboardOverride,
    },
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::id_as_i64::GetI64,
};

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
    /// If the original message is edted, whether to also update the content of the starboard message.
    #[command(rename = "link-edits")]
    link_edits: Option<bool>,
    /// How much XP each upvote on this starboard counts for.
    #[command(rename = "xp-multiplier", min_value = -10, max_value = 10)]
    xp_multiplier: Option<f64>,
    /// Whether to enable the per-user vote cooldown.
    #[command(rename = "cooldown-enabled")]
    cooldown_enabled: Option<bool>,
    /// The size of the cooldown (e.x. "5/6" means 5 votes per 6 seconds).
    cooldown: Option<String>,
}

impl EditBehavior {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let ov = match StarboardOverride::get(&ctx.bot.pool, guild_id.get_i64(), &self.name).await?
        {
            None => {
                ctx.respond_str("No override with that name was found.", true)
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
        if let Some(val) = self.xp_multiplier {
            let val = val.to_string().parse().unwrap();
            if let Err(why) = validation::starboard_settings::validate_xp_multiplier(val) {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            settings.xp_multiplier = Some(val);
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

        StarboardOverride::update_settings(&ctx.bot.pool, ov.id, settings).await?;
        ctx.respond_str(
            &format!("Updated settings for override '{}'.", self.name),
            false,
        )
        .await?;

        Ok(())
    }
}
