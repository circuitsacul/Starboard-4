use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    channel::message::Embed,
    id::{marker::GuildMarker, Id},
};

use crate::{
    client::bot::StarboardBot,
    concat_format,
    core::emoji::{EmojiCommon, SimpleEmoji},
    database::AutoStarChannel,
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::{
        embed,
        id_as_i64::GetI64,
        views::select_paginator::{SelectPaginatorBuilder, SelectPaginatorPageBuilder},
    },
};

#[derive(CreateCommand, CommandModel)]
#[command(name = "view", desc = "View your autostar channels.")]
pub struct ViewAutoStarChannels {
    /// The name of the autostar channel to view. Leave blank to show all.
    #[command(autocomplete = true)]
    name: Option<String>,
}

impl ViewAutoStarChannels {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();
        let bot = ctx.bot.clone();

        let asc = AutoStarChannel::list_by_guild(&ctx.bot.pool, guild_id_i64).await?;

        if asc.is_empty() {
            ctx.respond_str("This server has no autostar channels.", true)
                .await?;
            return Ok(());
        }

        let asc = AutoStarChannel::list_by_guild(&ctx.bot.pool, guild_id_i64).await?;

        if asc.is_empty() {
            ctx.respond_str("This server has no autostar channels.", true)
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
                label.push_str(" (premium-locked)");
            }

            let emb = autostar_embed(&bot, guild_id, a).await?;

            let page = SelectPaginatorPageBuilder::new(label).add_embed(emb);
            paginator = paginator.add_page(page);
        }

        paginator.current(current).build().run().await?;

        Ok(())
    }
}

async fn autostar_embed(
    bot: &StarboardBot,
    guild_id: Id<GuildMarker>,
    asc: AutoStarChannel,
) -> StarboardResult<Embed> {
    let emojis = Vec::<SimpleEmoji>::from_stored(asc.emojis).into_readable(bot, guild_id);
    let max_chars = asc
        .max_chars
        .map(|v| v.to_string())
        .unwrap_or_else(|| "none".to_string());

    let note = if asc.premium_locked {
        concat!(
            "This autostar channel is locked because it exceeds the non-premium ",
            "limit.\n\n"
        )
    } else {
        ""
    };

    let asc_settings = concat_format!(
        "{}" <- note;
        "This autostar channel is in <#{}>.\n\n" <- asc.channel_id;
        "emojis: {}\n" <- emojis;
        "min-chars: {}\n" <- asc.min_chars;
        "max-chars: {}\n" <- max_chars;
        "require-image: {}\n" <- asc.require_image;
        "delete-invalid: {}" <- asc.delete_invalid;
    );

    let emb = embed::build()
        .title(format!("Autostar Channel '{}'", asc.name))
        .description(asc_settings)
        .build();

    Ok(emb)
}
