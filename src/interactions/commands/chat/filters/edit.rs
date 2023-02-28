use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption};

use crate::{
    database::models::{filter::Filter, filter_group::FilterGroup},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::id_as_i64::GetI64,
};

#[derive(CreateOption, CommandOption)]
pub enum UserBotRequirement {
    #[option(name = "User must be a bot", value = 0)]
    MustBeBot,
    #[option(name = "User must not be a bot", value = 1)]
    MustBeHuman,
    #[option(name = "Disabled", value = 2)]
    Disabled,
}

impl From<UserBotRequirement> for Option<bool> {
    fn from(val: UserBotRequirement) -> Self {
        match val {
            UserBotRequirement::MustBeBot => Some(true),
            UserBotRequirement::MustBeHuman => Some(false),
            UserBotRequirement::Disabled => None,
        }
    }
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "edit", desc = "Edit a filters conditions.")]
pub struct Edit {
    /// The name of the filter group containing the filter to be edited.
    #[command(autocomplete = true)]
    group: String,
    /// The position of the filter to edit.
    #[command(min_value = 1, max_value = 1_000)]
    position: i64,

    // general info
    /// If true and this filter passes, the entire filter groups passes.
    #[command(rename = "instant-pass")]
    instant_pass: Option<bool>,
    /// If true and this filter fails, then the entire filter group fails.
    #[command(rename = "instant-fail")]
    instant_fail: Option<bool>,

    // default context
    /// Require that the user/author has all of these roles.
    #[command(rename = "user-has-all-of")]
    user_has_all_of: Option<String>,
    /// Require that the user/author has at least one of these roles.
    #[command(rename = "user-has-some-of")]
    user_has_some_of: Option<String>,
    /// Require that the user/author is missing all of these roles.
    #[command(rename = "user-missing-all-of")]
    user_missing_all_of: Option<String>,
    /// Require that the user/author is missing at least one of these roles.
    #[command(rename = "user-missing-some-of")]
    user_missing_some_of: Option<String>,
    /// Require that the user is or is not a bot.
    #[command(rename = "user-is-bot")]
    user_is_bot: Option<UserBotRequirement>,

    // message context
    /// Require that the message was sent in one of these channels.
    #[command(rename = "in-channel")]
    in_channel: Option<String>,
    /// Require that the message was not sent in one of these channels.
    #[command(rename = "not-in-channel")]
    not_in_channel: Option<String>,
    /// Require that the message was sent in one of these channels or their sub-channels.
    #[command(rename = "in-channel-or-sub-channels")]
    in_channel_or_sub_channels: Option<String>,
    /// Require that the message was not sent in one of these channels or their sub-channels.
    #[command(rename = "not-in-channel-or-sub-channels")]
    not_in_channel_or_sub_channels: Option<String>,
    /// Require that the message has at least this many attachments.
    #[command(rename = "min-attachments")]
    min_attachments: Option<String>,
    /// Require that the message have at most this many attachments.
    #[command(rename = "max-attachments")]
    max_attachments: Option<String>,
    /// Require that the message be at least this many characters long.
    #[command(rename = "min-length")]
    min_lengths: Option<i64>,
    /// Require that the message be at most this many characters long.
    #[command(rename = "max-length")]
    max_lengths: Option<i64>,
    /// (Premium) Require that the message match this regex. Use `.*` to disable.
    matches: Option<String>,
    /// (Premium) Require that the message not match this regex. Use `.*` to disable.
    #[command(rename = "not-matches")]
    not_matches: Option<String>,

    // vote context
    /// Require that the voter has all of these roles.
    #[command(rename = "voter-has-all-of")]
    voter_has_all_of: Option<String>,
    /// Require that the voter has at least one of these roles.
    #[command(rename = "voter-has-some-of")]
    voter_has_some_of: Option<String>,
    /// Require that the voter is missing all of these roles.
    #[command(rename = "voter-missing-all-of")]
    voter_missing_all_of: Option<String>,
    /// Require that the voter is missing at least one of these roles.
    #[command(rename = "voter-missing-some-of")]
    voter_missing_some_of: Option<String>,
    /// Require that the message being voted on is over a certain age.
    #[command(rename = "older-than")]
    older_than: Option<String>,
    /// Require that the message being voted on is under a certain age.
    #[command(rename = "newer-than")]
    newer_than: Option<String>,
}

impl Edit {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let Some(group) = FilterGroup::get_by_name(&ctx.bot.pool, guild_id, &self.group).await? else {
            ctx.respond_str(&format!("No filter group named '{}' exists.", self.group), true).await?;
            return Ok(());
        };

        let Some(mut filter) = Filter::get_by_position(&ctx.bot.pool, group.id, self.position as i16).await? else {
            ctx.respond_str(&format!("No filter for group '{}' at {} exists.", self.group, self.position), true).await?;
            return Ok(());
        };

        ctx.respond_str(&format!("{}{}", group.name, filter.position), true)
            .await?;

        // general info
        if let Some(val) = self.instant_pass {
            filter.instant_pass = val;
        }
        if let Some(val) = self.instant_fail {
            filter.instant_fail = val;
        }

        filter.update_settings(&ctx.bot.pool).await?;

        Ok(())
    }
}
