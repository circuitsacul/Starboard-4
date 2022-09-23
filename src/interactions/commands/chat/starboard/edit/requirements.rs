use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    core::emoji::{EmojiCommon, SimpleEmoji},
    database::{
        validation::{self, time_delta::parse_time_delta},
        Starboard,
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
    /// The starboard to edit.
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
        let mut starboard =
            match Starboard::get_by_name(&ctx.bot.pool, &self.name, unwrap_id!(guild_id)).await? {
                None => {
                    ctx.respond_str("No starboard with that name was found.", true)
                        .await?;
                    return Ok(());
                }
                Some(starboard) => starboard,
            };

        if let Some(val) = self.required {
            let val = val as i16;
            if let Err(why) = validation::starboard_settings::validate_required(
                val,
                starboard.settings.required_remove,
            ) {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            starboard.settings.required = val;
        }
        if let Some(val) = self.required_remove {
            let val = val as i16;
            if let Err(why) = validation::starboard_settings::validate_required_remove(
                val,
                starboard.settings.required,
            ) {
                ctx.respond_str(&why, true).await?;
                return Ok(());
            }
            starboard.settings.required_remove = val;
        }
        if let Some(val) = self.upvote_emojis {
            let emojis = Vec::<SimpleEmoji>::from_user_input(val, &ctx.bot, guild_id).into_stored();
            if let Err(why) = validation::starboard_settings::validate_vote_emojis(
                &emojis,
                &starboard.settings.downvote_emojis,
            ) {
                ctx.respond_str(why, true).await?;
                return Ok(());
            }
            starboard.settings.upvote_emojis = emojis;

            // delete cached value
            ctx.bot
                .cache
                .guild_vote_emojis
                .remove(&unwrap_id!(guild_id));
        }
        if let Some(val) = self.downvote_emojis {
            let emojis = Vec::<SimpleEmoji>::from_user_input(val, &ctx.bot, guild_id).into_stored();
            if let Err(why) = validation::starboard_settings::validate_vote_emojis(
                &starboard.settings.upvote_emojis,
                &emojis,
            ) {
                ctx.respond_str(why, true).await?;
                return Ok(());
            }
            starboard.settings.downvote_emojis = emojis;

            // delete cached value
            ctx.bot
                .cache
                .guild_vote_emojis
                .remove(&unwrap_id!(guild_id));
        }
        if let Some(val) = self.self_vote {
            starboard.settings.self_vote = val;
        }
        if let Some(val) = self.allow_bots {
            starboard.settings.allow_bots = val;
        }
        if let Some(val) = self.require_image {
            starboard.settings.require_image = val;
        }
        if let Some(val) = self.older_than {
            let delta = match parse_time_delta(&val) {
                Err(why) => {
                    ctx.respond_str(&why, true).await?;
                    return Ok(());
                }
                Ok(delta) => delta,
            };
            starboard.settings.older_than = delta;
        }
        if let Some(val) = self.newer_than {
            let delta = match parse_time_delta(&val) {
                Err(why) => {
                    ctx.respond_str(&why, true).await?;
                    return Ok(());
                }
                Ok(delta) => delta,
            };
            starboard.settings.newer_than = delta;
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
