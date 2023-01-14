use futures::{stream::BoxStream, TryStreamExt};
use sqlx::{Postgres, QueryBuilder};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::application_command::InteractionChannel,
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component,
    },
    id::{marker::MessageMarker, Id},
    user::User,
};

use crate::{
    core::embedder::builder::BuiltStarboardEmbed,
    database::{Message, Starboard, StarboardMessage},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::{CommandCtx, ComponentCtx},
    utils::{id_as_i64::GetI64, views::wait_for::wait_for_button},
};

use super::random::{get_config, get_embedder, get_post_query};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "moststarred",
    desc = "Shows the most starred messages in this server.",
    dm_permission = false
)]
pub struct Moststarred {
    /// The starboard to show the most starred messages for.
    #[command(autocomplete = true)]
    starboard: String,

    /// Only show messages with at least this many points.
    #[command(rename = "min-points", max_value = 32767, min_value = -32767)]
    min_points: Option<i64>,
    /// Only show messages with at most this many points.
    #[command(rename = "max-points", max_value = 32767, min_value = -32767)]
    max_points: Option<i64>,
    /// Only show messages that were sent in this channel.
    channel: Option<InteractionChannel>,
    /// Only show messages sent by this user.
    author: Option<User>,
    /// Whether to allow messages from NSFW starboards.
    #[command(rename = "allow-nsfw")]
    allow_nsfw: Option<bool>,
}

impl Moststarred {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let guild_id_i64 = guild_id.get_i64();

        let Some(sb) = Starboard::get_by_name(&ctx.bot.pool, &self.starboard, guild_id_i64).await? else {
            ctx.respond_str(&format!("Starboard '{}' does not exist.", self.starboard), true).await?;
            return Ok(());
        };

        if sb.settings.private {
            ctx.respond_str("That starboard is private.", true).await?;
            return Ok(());
        }

        let allow_nsfw = self.allow_nsfw.unwrap_or(false);

        if allow_nsfw {
            let nsfw = ctx
                .bot
                .cache
                .fog_channel_nsfw(&ctx.bot, guild_id, ctx.interaction.channel_id.unwrap())
                .await?;
            if !nsfw.unwrap() {
                ctx.respond_str(
                    "This channel isn't NSFW, so you can't allow NSFW messages.",
                    true,
                )
                .await?;
                return Ok(());
            }
        }

        let mut query = get_post_query(
            sb.id,
            allow_nsfw,
            self.channel.map(|ch| ch.id.get_i64()),
            self.author.map(|user| user.id.get_i64()),
            self.min_points.map(|v| v as i16),
            self.max_points.map(|v| v as i16),
        );
        query.push(" ORDER BY last_known_point_count DESC");

        scrolling_paginator(ctx, query, sb).await?;

        Ok(())
    }
}

fn components(current_page: usize, done: bool) -> Vec<Component> {
    let buttons = vec![
        Component::Button(Button {
            custom_id: Some("moststarred_scroller::page_number".to_string()),
            disabled: true,
            emoji: None,
            label: Some(current_page.to_string()),
            style: ButtonStyle::Secondary,
            url: None,
        }),
        Component::Button(Button {
            custom_id: Some("moststarred_scroller::next".to_string()),
            disabled: done,
            emoji: None,
            label: Some("Next".to_string()),
            style: ButtonStyle::Secondary,
            url: None,
        }),
    ];

    vec![Component::ActionRow(ActionRow {
        components: buttons,
    })]
}

async fn scrolling_paginator(
    mut ctx: CommandCtx,
    mut query: QueryBuilder<'_, Postgres>,
    starboard: Starboard,
) -> StarboardResult<()> {
    let user_id = ctx.interaction.author_id().unwrap();

    let bot = ctx.bot.clone();
    let mut messages: BoxStream<'_, sqlx::Result<StarboardMessage>> =
        query.build_query_as().fetch(&bot.pool);

    let mut btn_ctx: Option<ComponentCtx> = None;
    let mut message_id: Option<Id<MessageMarker>> = None;
    let mut current_page: usize = 0;

    loop {
        let Some(next_sb_message) = messages.try_next().await? else {
            break;
        };

        current_page += 1;

        // built message
        let orig_msg = Message::get(&ctx.bot.pool, next_sb_message.message_id)
            .await?
            .unwrap();
        let config = get_config(&ctx.bot, starboard.clone(), orig_msg.channel_id).await?;
        let embedder = get_embedder(&ctx.bot, &config, orig_msg, next_sb_message).await?;
        let Some(embedder) = embedder else {
            continue;
        };

        let built = embedder.build(false, false);
        let BuiltStarboardEmbed::Full(built) = built else {
            unreachable!("didn't get full embed");
        };

        // respond
        let data = ctx
            .build_resp()
            .components(components(current_page, false))
            .embeds(built.embeds)
            .content(built.top_content)
            .build();

        if let Some(btn_ctx) = &mut btn_ctx {
            btn_ctx.edit(data).await?;
        } else {
            let resp = ctx.respond(data).await?.model().await?;
            message_id = Some(resp.id);
        }

        // wait for component interaction
        btn_ctx = wait_for_button(
            ctx.bot.clone(),
            &["moststarred_scroller::next"],
            message_id.unwrap(),
            user_id,
        )
        .await;

        if btn_ctx.is_none() {
            break;
        }
    }

    if let Some(btn_ctx) = &mut btn_ctx {
        btn_ctx
            .edit(
                btn_ctx
                    .build_resp()
                    .components(components(current_page, true))
                    .build(),
            )
            .await?;
    } else if message_id.is_some() {
        let i = ctx.bot.interaction_client().await;
        i.update_response(&ctx.interaction.token)
            .components(Some(&components(current_page, true)))?
            .await?;
    } else {
        ctx.respond_str("Nothing to show.", true).await?;
    }

    Ok(())
}
