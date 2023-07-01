use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    channel::message::Embed,
    id::{marker::GuildMarker, Id},
};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{
    client::bot::StarboardBot,
    concat_format,
    core::emoji::{EmojiCommon, SimpleEmoji},
    database::{
        models::{
            autostar_channel_filter_group::AutostarChannelFilterGroup, filter_group::FilterGroup,
        },
        AutoStarChannel,
    },
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    translations::Lang,
    utils::{
        embed,
        id_as_i64::GetI64,
        views::select_paginator::{SelectPaginatorBuilder, SelectPaginatorPageBuilder},
    },
};

locale_func!(autostar_view);
locale_func!(autostar_view_option_name);

#[derive(CreateCommand, CommandModel)]
#[command(
    name = "view",
    desc = "View your autostar channels.",
    desc_localizations = "autostar_view"
)]
pub struct ViewAutoStarChannels {
    /// The name of the autostar channel to view. Leave blank to show all.
    #[command(autocomplete = true, desc_localizations = "autostar_view_option_name")]
    name: Option<String>,
}

impl ViewAutoStarChannels {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();
        let bot = ctx.bot.clone();
        let lang = ctx.user_lang();

        let asc = AutoStarChannel::list_by_guild(&ctx.bot.pool, guild_id_i64).await?;

        if asc.is_empty() {
            ctx.respond_str(ctx.user_lang().autostar_view_no_autostar_channels(), true)
                .await?;
            return Ok(());
        }

        let asc = AutoStarChannel::list_by_guild(&ctx.bot.pool, guild_id_i64).await?;

        if asc.is_empty() {
            ctx.respond_str(ctx.user_lang().autostar_view_no_autostar_channels(), true)
                .await?;
            return Ok(());
        }

        let mut paginator = SelectPaginatorBuilder::new(ctx);
        let mut current = 0;

        for (idx, a) in asc.into_iter().enumerate() {
            if self.name.as_ref() == Some(&a.name) {
                current = idx;
            }
            let mut label = a.name.clone();
            if a.premium_locked {
                label.push(' ');
                label.push_str(lang.premium_locked_aside());
            }

            let emb = autostar_embed(&bot, guild_id, a, lang).await?;

            let page = SelectPaginatorPageBuilder::new(label).add_embed(emb);
            paginator = paginator.add_page(page);
        }

        paginator.current(current).build().run().await?;

        Ok(())
    }
}

async fn filters_str(bot: &StarboardBot, asc_id: i32, lang: Lang) -> StarboardResult<String> {
    let filter_group_ids =
        AutostarChannelFilterGroup::list_by_autostar_channel(&bot.pool, asc_id).await?;
    let filter_group_ids = filter_group_ids.into_iter().map(|f| f.filter_group_id);
    let mut filters = Vec::new();
    for id in filter_group_ids {
        let filter_group = FilterGroup::get(&bot.pool, id).await?;
        filters.push(filter_group.name);
    }

    let mut filters = filters.join(", ");
    if filters.is_empty() {
        filters = lang.autostar_view_filters_none().to_string();
    }
    Ok(lang.autostar_view_filters_info(filters))
}

async fn autostar_embed(
    bot: &StarboardBot,
    guild_id: Id<GuildMarker>,
    asc: AutoStarChannel,
    lang: Lang,
) -> StarboardResult<Embed> {
    let emojis = Vec::<SimpleEmoji>::from_stored(asc.emojis).into_readable(bot, guild_id);
    let max_chars = asc
        .max_chars
        .map(|v| v.to_string())
        .unwrap_or_else(|| lang.disabled().to_string());

    let note = if asc.premium_locked {
        lang.premium_locked_autostar_info()
    } else {
        ""
    };

    let asc_settings = concat_format!(
        "{}" <- note;
        "{}\n\n" <- lang.autostar_view_channel(asc.channel_id);
        "emojis: {}\n" <- emojis;
        "min-chars: {}\n" <- asc.min_chars;
        "max-chars: {}\n" <- max_chars;
        "require-image: {}\n" <- asc.require_image;
        "delete-invalid: {}" <- asc.delete_invalid;
    );

    let emb = embed::build()
        .title(lang.autostar_view_title(asc.name))
        .description(asc_settings)
        .field(EmbedFieldBuilder::new(
            lang.filters_title(),
            filters_str(bot, asc.id, lang).await?,
        ))
        .build();

    Ok(emb)
}
