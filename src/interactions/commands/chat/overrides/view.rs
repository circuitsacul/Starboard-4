use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    channel::message::Embed,
    id::{marker::GuildMarker, Id},
};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{
    client::bot::StarboardBot,
    core::starboard::config::StarboardConfig,
    database::{Starboard, StarboardOverride},
    errors::StarboardResult,
    get_guild_id,
    interactions::{commands::format_settings::format_settings, context::CommandCtx},
    locale_func,
    translations::Lang,
    utils::{
        embed,
        id_as_i64::GetI64,
        views::select_paginator::{SelectPaginatorBuilder, SelectPaginatorPageBuilder},
    },
};

locale_func!(overrides_view);
locale_func!(overrides_view_option_name);

#[derive(CreateCommand, CommandModel)]
#[command(
    name = "view",
    desc = "View your overrides.",
    desc_localizations = "overrides_view"
)]
pub struct ViewOverride {
    /// The name of the override to view. Leave blank to show all.
    #[command(autocomplete = true, desc_localizations = "overrides_view_option_name")]
    name: Option<String>,
}

impl ViewOverride {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();
        let bot = ctx.bot.clone();
        let lang = ctx.user_lang();

        let overrides = StarboardOverride::list_by_guild(&ctx.bot.pool, guild_id_i64).await?;
        if overrides.is_empty() {
            ctx.respond_str(lang.overrides_view_none(), true).await?;
            return Ok(());
        }

        let mut paginator = SelectPaginatorBuilder::new(ctx);
        let mut current = 0;

        for (idx, ov) in overrides.into_iter().enumerate() {
            if self.name.as_ref() == Some(&ov.name) {
                current = idx;
            }

            let sb = Starboard::get(&bot.pool, ov.starboard_id).await?.unwrap();

            let label = ov.name.clone();
            let description = lang.overrides_view_slug(
                ov.channel_ids.len(),
                ov.overrides.as_object().unwrap().len(),
                sb.name,
            );

            let page = SelectPaginatorPageBuilder::new(label)
                .description(description)
                .add_embed(override_embed(&bot, guild_id, ov, lang).await?);
            paginator = paginator.add_page(page);
        }

        paginator.current(current).build().run().await?;

        Ok(())
    }
}

async fn override_embed(
    bot: &StarboardBot,
    guild_id: Id<GuildMarker>,
    ov: StarboardOverride,
    lang: Lang,
) -> StarboardResult<Embed> {
    let name = ov.name.clone();
    let sb = Starboard::get(&bot.pool, ov.starboard_id).await?.unwrap();

    let channels: Vec<_> = ov.channel_ids.iter().map(|id| format!("<#{id}>")).collect();
    let channels = channels.join(", ");
    let config = StarboardConfig::new(sb, &[], vec![ov])?;
    let pretty = format_settings(bot, guild_id, &config).await?;

    let embed = embed::build()
        .title(lang.override_title(name))
        .description(lang.overrides_view_description(channels, &config.starboard.name))
        .field(
            EmbedFieldBuilder::new(lang.sb_option_category_requirements(), pretty.requirements)
                .inline()
                .build(),
        )
        .field(
            EmbedFieldBuilder::new(lang.sb_option_category_behavior(), pretty.behavior)
                .inline()
                .build(),
        )
        .field(
            EmbedFieldBuilder::new(lang.sb_option_category_style(), pretty.style)
                .inline()
                .build(),
        )
        .field(
            EmbedFieldBuilder::new(lang.sb_option_category_embed(), pretty.embed)
                .inline()
                .build(),
        )
        .field(EmbedFieldBuilder::new(lang.sb_option_category_regex(), pretty.regex).build())
        .field(EmbedFieldBuilder::new(lang.filters_title(), pretty.filters))
        .build();

    Ok(embed)
}
