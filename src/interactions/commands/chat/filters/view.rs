use std::time::Duration;
use thousands::Separable;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::channel::message::Embed;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::{
    constants,
    core::premium::is_premium::is_guild_premium,
    database::models::{filter::Filter, filter_group::FilterGroup},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    translations::Lang,
    utils::{
        id_as_i64::GetI64,
        views::select_paginator::{SelectPaginatorBuilder, SelectPaginatorPageBuilder},
    },
};

locale_func!(filters_view);
locale_func!(filters_view_option_group);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "view",
    desc = "View filter groups and filters for this server.",
    desc_localizations = "filters_view"
)]
pub struct View {
    /// The filter group to view.
    #[command(autocomplete = true, desc_localizations = "filters_view_option_group")]
    group: Option<String>,
}

impl View {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();
        let lang = ctx.user_lang();

        let groups = FilterGroup::list_by_guild(&ctx.bot.pool, guild_id).await?;

        if groups.is_empty() {
            ctx.respond_str(&lang.filters_view_no_groups(constants::DOCS_FILTERS), true)
                .await?;
            return Ok(());
        }

        let premium = is_guild_premium(&ctx.bot, guild_id, true).await?;

        let bot = ctx.bot.clone();
        let mut paginator = SelectPaginatorBuilder::new(ctx);
        let mut start = 0;
        for (x, group) in groups.into_iter().enumerate() {
            if Some(&group.name) == self.group.as_ref() {
                start = x;
            }
            let embeds = group_embed(&bot.pool, &group, premium, lang).await?;
            let page =
                SelectPaginatorPageBuilder::new(lang.filter_group_title(group.name)).embeds(embeds);
            paginator = paginator.add_page(page);
        }

        paginator.current(start).build().run().await
    }
}

async fn group_embed(
    pool: &sqlx::PgPool,
    group: &FilterGroup,
    premium: bool,
    lang: Lang,
) -> StarboardResult<Vec<Embed>> {
    let mut ret = Vec::new();
    let emb = EmbedBuilder::new()
        .color(constants::EMBED_DARK_BG)
        .title(lang.filter_group_title(&group.name))
        .description(lang.filters_view_read_docs(constants::DOCS_FILTERS))
        .build();
    ret.push(emb);

    let filters = Filter::list_by_filter(pool, group.id).await?;
    if filters.is_empty() {
        let emb = EmbedBuilder::new()
            .color(constants::EMBED_DARK_BG)
            .description(lang.filters_view_group_no_filters())
            .build();
        ret.push(emb);
    }

    for filter in filters {
        ret.push(filter_embed(filter, premium, lang));
    }

    Ok(ret)
}

fn format_roles(role_ids: &[i64], lang: Lang) -> String {
    if role_ids.is_empty() {
        lang.filters_view_condition_no_roles().to_string()
    } else {
        role_ids
            .iter()
            .map(|r| format!("<@&{r}>"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn format_channels(channel_ids: &[i64], lang: Lang) -> String {
    if channel_ids.is_empty() {
        lang.filters_view_condition_no_channels().to_string()
    } else {
        channel_ids
            .iter()
            .map(|c| format!("<#{c}>"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn filter_embed(filter: Filter, premium: bool, lang: Lang) -> Embed {
    let mut default_context = Vec::new();
    let mut message_context = Vec::new();
    let mut vote_context = Vec::new();

    // default context
    if let Some(val) = filter.user_has_all_of {
        let desc = lang.filters_view_user_has_all_of(format_roles(&val, lang));
        default_context.push(desc);
    }
    if let Some(val) = filter.user_has_some_of {
        let desc = lang.filters_view_user_has_some_of(format_roles(&val, lang));
        default_context.push(desc);
    }
    if let Some(val) = filter.user_missing_all_of {
        let desc = lang.filters_view_user_missing_all_of(format_roles(&val, lang));
        default_context.push(desc);
    }
    if let Some(val) = filter.user_missing_some_of {
        let desc = lang.filters_view_user_missing_some_of(format_roles(&val, lang));
        default_context.push(desc);
    }
    if let Some(val) = filter.user_is_bot {
        let desc = if val {
            lang.filters_view_user_must_be_bot()
        } else {
            lang.filters_view_user_must_be_human()
        };
        default_context.push(desc.to_string());
    }

    // message context
    if let Some(val) = filter.in_channel {
        let desc = lang.filters_view_in_channel(format_channels(&val, lang));
        message_context.push(desc);
    }
    if let Some(val) = filter.not_in_channel {
        let desc = lang.filters_view_not_in_channel(format_channels(&val, lang));
        message_context.push(desc);
    }
    if let Some(val) = filter.in_channel_or_sub_channels {
        let desc = lang.filters_view_in_channel_or_sub_channels(format_channels(&val, lang));
        message_context.push(desc);
    }
    if let Some(val) = filter.not_in_channel_or_sub_channels {
        let desc = lang.filters_view_not_in_channel_or_sub_channels(format_channels(&val, lang));
        message_context.push(desc);
    }
    if let Some(val) = filter.min_attachments {
        let desc = lang.filters_view_min_attachments(val);
        message_context.push(desc);
    }
    if let Some(val) = filter.max_attachments {
        let desc = lang.filters_view_max_attachments(val);
        message_context.push(desc);
    }
    if let Some(val) = filter.min_length {
        let desc = lang.filters_view_min_length(val.separate_with_commas());
        message_context.push(desc);
    }
    if let Some(val) = filter.max_length {
        let desc = lang.filters_view_max_length(val.separate_with_commas());
        message_context.push(desc);
    }
    if let Some(val) = filter.matches {
        let mut desc = lang.filters_view_matches(val);
        if !premium {
            desc.push_str(lang.filters_view_regex_not_applied());
        }
        message_context.push(desc);
    }
    if let Some(val) = filter.not_matches {
        let mut desc = lang.filters_view_not_matches(val);
        if !premium {
            desc.push_str(lang.filters_view_regex_not_applied());
        }
        message_context.push(desc);
    }

    // voter context
    if let Some(val) = filter.voter_has_all_of {
        let desc = lang.filters_view_voter_has_all_of(format_roles(&val, lang));
        vote_context.push(desc);
    }
    if let Some(val) = filter.voter_has_some_of {
        let desc = lang.filters_view_voter_has_some_of(format_roles(&val, lang));
        vote_context.push(desc);
    }
    if let Some(val) = filter.voter_missing_all_of {
        let desc = lang.filters_view_voter_missing_all_of(format_roles(&val, lang));
        vote_context.push(desc);
    }
    if let Some(val) = filter.voter_missing_some_of {
        let desc = lang.filters_view_voter_missing_some_of(format_roles(&val, lang));
        vote_context.push(desc);
    }
    if let Some(val) = filter.older_than {
        let desc = lang
            .filters_view_older_than(humantime::format_duration(Duration::from_secs(val as u64)));
        vote_context.push(desc);
    }
    if let Some(val) = filter.newer_than {
        let desc = lang
            .filters_view_newer_than(humantime::format_duration(Duration::from_secs(val as u64)));
        vote_context.push(desc);
    }

    let mut desc = String::new();
    desc.push_str(&format!(
        "instant-pass: {}\ninstant-fail: {}",
        filter.instant_pass, filter.instant_fail
    ));

    let mut emb = EmbedBuilder::new().color(constants::EMBED_DARK_BG);

    let mut has_conditions = false;

    if !default_context.is_empty() {
        has_conditions = true;
        emb = emb.field(EmbedFieldBuilder::new(
            lang.filters_default_context_title(),
            default_context.join("\n\n"),
        ));
    }
    if !message_context.is_empty() {
        has_conditions = true;
        emb = emb.field(EmbedFieldBuilder::new(
            lang.filters_message_context_title(),
            message_context.join("\n\n"),
        ));
    }
    if !vote_context.is_empty() {
        has_conditions = true;
        emb = emb.field(EmbedFieldBuilder::new(
            lang.filters_vote_context_title(),
            vote_context.join("\n\n"),
        ));
    }

    if !has_conditions {
        desc.push_str(lang.filters_view_no_conditions());
    }

    emb = emb
        .description(desc)
        .title(lang.filter_title(filter.position));

    emb.build()
}
