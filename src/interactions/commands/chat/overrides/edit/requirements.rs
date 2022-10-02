use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    core::{
        emoji::{EmojiCommon, SimpleEmoji},
        starboard::config::StarboardConfig,
    },
    database::{
        validation::{self, time_delta::parse_time_delta},
        Starboard, StarboardOverride,
    },
    get_guild_id,
    interactions::context::CommandCtx,
    unwrap_id,
};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "requirements",
    desc = "Edit the requirements for messages to appear on the starboard."
)]
pub struct EditRequirements {
    /// The override to edit.
    #[command(autocomplete = true)]
    name: String,

    /// The number of upvotes a message needs.
    #[command(min_value=-1, max_value=500)]
    required: Option<i64>,
    /// If a message is already on the starboard, how few points the message can have before it is removed.
    #[command(rename="required-remove", min_value=-500, max_value=490)]
    required_remove: Option<i64>,
    /// The emojis that can be used to upvote a post. Use 'none' to remove all.
    #[command(rename = "upvote-emojis")]
    upvote_emojis: Option<String>,
    /// The emojis that can be used to downvote a post. Use 'none' to remove all.
    #[command(rename = "downvote-emojis")]
    downvote_emojis: Option<String>,
    /// Whether to allow users to vote on their own posts.
    #[command(rename = "self-vote")]
    self_vote: Option<bool>,
    /// Whether to allow bot messages to be on the starboard.
    #[command(rename = "allow-bots")]
    allow_bots: Option<bool>,
    /// Whether to require posts to have an image to appear on the starboard.
    #[command(rename = "require-image")]
    require_image: Option<bool>,
    /// How old a post must be in order for it to be voted on (e.g. "1 hour"). Use 0 to disable.
    #[command(rename = "older-than")]
    older_than: Option<String>,
    /// How new a post must be in order for it to be voted on (e.g. "1 hour"). Use 0 to disable.
    #[command(rename = "newer-than")]
    newer_than: Option<String>,
}

impl EditRequirements {
    pub async fn callback(self, mut ctx: CommandCtx) -> anyhow::Result<()> {
        let guild_id = get_guild_id!(ctx);
        let ov =
            match StarboardOverride::get(&ctx.bot.pool, unwrap_id!(guild_id), &self.name).await? {
                None => {
                    ctx.respond_str("No override with that name was found.", true)
                        .await?;
                    return Ok(());
                }
                Some(starboard) => starboard,
            };
        let (ov, resolved) = {
            let starboard = Starboard::get(&ctx.bot.pool, ov.starboard_id)
                .await?
                .unwrap();
            let mut resolved = StarboardConfig::new(starboard, vec![ov]).unwrap();

            (resolved.overrides.remove(0), resolved.resolved)
        };
        let mut settings = ov.get_overrides()?;

        if let Some(val) = self.required {
            let val = val as i16;
            if let Err(why) =
                validation::starboard_settings::validate_required(val, resolved.required_remove)
            {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            settings.required = Some(val);
        }
        if let Some(val) = self.required_remove {
            let val = val as i16;
            if let Err(why) =
                validation::starboard_settings::validate_required_remove(val, resolved.required)
            {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            settings.required_remove = Some(val);
        }
        if let Some(val) = self.upvote_emojis {
            let emojis = Vec::<SimpleEmoji>::from_user_input(val, &ctx.bot, guild_id).into_stored();
            if let Err(why) = validation::starboard_settings::validate_vote_emojis(
                &emojis,
                &resolved.downvote_emojis,
            ) {
                ctx.respond_str(why, true).await?;
                return Ok(());
            }
            settings.upvote_emojis = Some(emojis);

            // delete cached value
            ctx.bot
                .cache
                .guild_vote_emojis
                .remove(&unwrap_id!(guild_id));
        }
        if let Some(val) = self.downvote_emojis {
            let emojis = Vec::<SimpleEmoji>::from_user_input(val, &ctx.bot, guild_id).into_stored();
            if let Err(why) = validation::starboard_settings::validate_vote_emojis(
                &resolved.upvote_emojis,
                &emojis,
            ) {
                ctx.respond_str(why, true).await?;
                return Ok(());
            }
            settings.downvote_emojis = Some(emojis);

            // delete cached value
            ctx.bot
                .cache
                .guild_vote_emojis
                .remove(&unwrap_id!(guild_id));
        }
        if let Some(val) = self.self_vote {
            settings.self_vote = Some(val);
        }
        if let Some(val) = self.allow_bots {
            settings.allow_bots = Some(val);
        }
        if let Some(val) = self.require_image {
            settings.require_image = Some(val);
        }
        if let Some(val) = self.older_than {
            let delta = match parse_time_delta(&val) {
                Err(why) => {
                    ctx.respond_str(&why, true).await?;
                    return Ok(());
                }
                Ok(delta) => delta,
            };
            settings.older_than = Some(delta);
        }
        if let Some(val) = self.newer_than {
            let delta = match parse_time_delta(&val) {
                Err(why) => {
                    ctx.respond_str(&why, true).await?;
                    return Ok(());
                }
                Ok(delta) => delta,
            };
            settings.newer_than = Some(delta);
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
