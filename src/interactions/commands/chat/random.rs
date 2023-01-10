use std::sync::Arc;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    client::bot::StarboardBot,
    core::{
        embedder::{builder::BuiltStarboardEmbed, Embedder},
        starboard::config::StarboardConfig,
    },
    database::{Message, Starboard, StarboardMessage, StarboardOverride},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::{id_as_i64::GetI64, into_id::IntoId},
};

fn get_post_query(starboard_id: i32) -> sqlx::QueryBuilder<'static, sqlx::Postgres> {
    let init_query = r#"
    SELECT * FROM starboard_messages
    WHERE EXISTS (
        SELECT * FROM messages
            WHERE message_id=starboard_messages.message_id
            AND trashed=false
    "#;
    let mut builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(init_query);

    // subquery
    builder.push(" AND is_nsfw=false");

    // outer query
    builder.push(") AND starboard_id=");
    builder.push_bind(starboard_id);

    builder
}

async fn get_config(
    bot: &StarboardBot,
    sb: Starboard,
    channel_id: i64,
) -> StarboardResult<StarboardConfig> {
    let overrides =
        StarboardOverride::list_by_starboard_and_channels(&bot.pool, sb.id, &[channel_id]).await?;
    Ok(StarboardConfig::new(sb, overrides).unwrap())
}

async fn get_embedder<'config, 'bot>(
    bot: &'bot StarboardBot,
    config: &'config StarboardConfig,
    orig_sql_msg: Message,
    msg: StarboardMessage,
) -> StarboardResult<Embedder<'config, 'bot>> {
    let orig_msg = bot
        .cache
        .fog_message(
            bot,
            orig_sql_msg.channel_id.into_id(),
            orig_sql_msg.message_id.into_id(),
        )
        .await?
        .unwrap();
    let orig_author = bot
        .cache
        .fog_user(bot, orig_sql_msg.author_id.into_id())
        .await?;

    let ref_msg = if let Some(ref_msg_id) = orig_msg.referenced_message {
        bot.cache
            .fog_message(bot, orig_sql_msg.channel_id.into_id(), ref_msg_id)
            .await?
    } else {
        None
    };

    let ref_msg_author = if let Some(ref_msg) = &ref_msg {
        bot.cache.fog_user(bot, ref_msg.author_id).await?
    } else {
        None
    };

    let embedder = Embedder {
        bot,
        points: msg.last_known_point_count as i32,
        config,
        orig_message: Some(orig_msg),
        orig_message_author: orig_author,
        referenced_message: ref_msg,
        referenced_message_author: ref_msg_author,
        orig_sql_message: Arc::new(orig_sql_msg),
    };

    Ok(embedder)
}

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "random",
    desc = "Get a random starboard post.",
    dm_permission = false
)]
pub struct RandomPost {
    /// The starboard to get a random post from.
    #[command(autocomplete = true)]
    starboard: String,
}

impl RandomPost {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let Some(sb) = Starboard::get_by_name(&ctx.bot.pool, &self.starboard, guild_id).await? else {
            ctx.respond_str(&format!("Starboard '{}' does not exist.", self.starboard), true).await?;
            return Ok(());
        };
        if sb.settings.private {
            ctx.respond_str(
                "That starboard is private, so you cannot get random messages from it.",
                true,
            )
            .await?;
            return Ok(());
        }

        let mut builder = get_post_query(sb.id);
        builder.push(" ORDER BY random()");
        let msg: Option<StarboardMessage> = builder
            .build_query_as()
            .fetch_optional(&ctx.bot.pool)
            .await?;
        let Some(msg) = msg else {
            ctx.respond_str("Nothing to show.", true).await?;
            return Ok(());
        };

        let orig_msg = Message::get(&ctx.bot.pool, msg.message_id).await?.unwrap();
        let config = get_config(&ctx.bot, sb, orig_msg.channel_id).await?;
        let embedder = get_embedder(&ctx.bot, &config, orig_msg, msg).await?;

        let built = embedder.build(false, false);
        let built = match built {
            BuiltStarboardEmbed::Partial(_) => unreachable!("didn't get full embed"),
            BuiltStarboardEmbed::Full(built) => built,
        };

        let data = ctx
            .build_resp()
            .content(built.top_content)
            .embeds(built.embeds)
            .build();
        ctx.respond(data).await?;

        Ok(())
    }
}
