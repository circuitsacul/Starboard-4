use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::channel::message::Embed;
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    constants,
    database::models::{filter::Filter, filter_group::FilterGroup},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::{
        id_as_i64::GetI64,
        views::select_paginator::{SelectPaginatorBuilder, SelectPaginatorPageBuilder},
    },
};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "view",
    desc = "View filter groups and filters for this server."
)]
pub struct View {
    /// The filter group to view.
    #[command(autocomplete = true)]
    group: Option<String>,
}

impl View {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let groups = FilterGroup::list_by_guild(&ctx.bot.pool, guild_id).await?;

        if groups.is_empty() {
            ctx.respond_str("This server does not have any filter groups.", true)
                .await?;
            return Ok(());
        }

        let bot = ctx.bot.clone();
        let mut paginator = SelectPaginatorBuilder::new(ctx);
        let mut start = 0;
        for (x, group) in groups.into_iter().enumerate() {
            if Some(&group.name) == self.group.as_ref() {
                start = x;
            }
            let embeds = group_embed(&bot.pool, &group).await?;
            let page =
                SelectPaginatorPageBuilder::new(format!("Filter '{}'", group.name)).embeds(embeds);
            paginator = paginator.add_page(page);
        }

        paginator.current(start).build().run().await
    }
}

async fn group_embed(pool: &sqlx::PgPool, group: &FilterGroup) -> StarboardResult<Vec<Embed>> {
    let mut ret = Vec::new();
    let emb = EmbedBuilder::new()
        .color(constants::EMBED_DARK_BG)
        .title(format!("Filter Group '{}'", group.name))
        .build();
    ret.push(emb);

    let filters = Filter::list_by_filter(pool, group.id).await?;
    if filters.is_empty() {
        let emb = EmbedBuilder::new()
            .color(constants::EMBED_DARK_BG)
            .description("This filter group has no filters.")
            .build();
        ret.push(emb);
    }

    for filter in filters {
        let emb = EmbedBuilder::new()
            .color(constants::EMBED_DARK_BG)
            .description(format!("Filter description. {}", filter.position))
            .build();
        ret.push(emb);
    }

    Ok(ret)
}
