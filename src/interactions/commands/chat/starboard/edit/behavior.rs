use lazy_static::lazy_static;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::{validation, Starboard},
    get_guild_id,
    interactions::commands::context::CommandCtx,
    unwrap_id,
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
    /// If true, prevents /random and /moststarred from pulling from this starboard.
    private: Option<bool>,
    /// How much XP each upvote on this starboard counts for.
    #[command(rename = "xp-multiplier", min_value = 10, max_value = 10)]
    xp_multiplier: Option<f64>,
    /// Whether to enable the per-user vote cooldown.
    #[command(rename = "cooldown-enabled")]
    cooldown_enabled: Option<bool>,
    /// The size of the cooldown (e.x. "5/6" means 5 votes per 6 seconds).
    cooldown: Option<String>,
}

impl EditBehavior {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);
        let mut starboard =
            match Starboard::get_by_name(&ctx.bot.pool, &self.name, unwrap_id!(guild_id)).await? {
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

        starboard.update_settings(&ctx.bot.pool).await?;
        ctx.respond_str(
            &format!("Updated settings for starboard '{}'.", self.name),
            false,
        )
        .await?;

        Ok(())
    }
}

fn parse_cooldown(inp: &str) -> Result<(i16, i16), String> {
    lazy_static! {
        static ref RE: regex::Regex =
            regex::Regex::new(r#"(?P<count>\d+).+?(?P<secs>\d+)"#).unwrap();
    }

    let found = match RE.captures(inp) {
        None => {
            return Err(concat!(
                "I couldn't parse the cooldown you passed. The ",
                "correct format is `capacity/period` (e.x. `5/6`)."
            )
            .to_string())
        }
        Some(found) => found,
    };

    let capacity = found.name("count").unwrap().as_str();
    let capacity: i16 = match capacity.parse() {
        Err(_) => return Err(format!("{capacity} is not a valid number.")),
        Ok(capacity) => capacity,
    };
    let period = found.name("secs").unwrap().as_str();
    let period: i16 = match period.parse() {
        Err(_) => return Err(format!("{period} is not a valid number.")),
        Ok(period) => period,
    };

    validation::starboard_settings::validate_cooldown(capacity, period)?;
    Ok((capacity, period))
}
