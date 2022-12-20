use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{
    concat_format,
    database::{Message, Starboard, StarboardMessage},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::{
        embed,
        id_as_i64::GetI64,
        message_link::{fmt_message_link, parse_message_link},
    },
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "info", desc = "Get info for a message.")]
pub struct Info {
    /// Link to the message to get info for.
    message: String,
}

impl Info {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let Some((_channel_id, message_id)) = parse_message_link(&self.message) else {
            ctx.respond_str("Invalid message link.", true).await?;
            return Ok(());
        };

        let Some(sql_msg) = Message::get_original(&ctx.bot.pool, message_id).await? else {
            ctx.respond_str(concat!(
                "I couldn't find that message. There are a few possible reasons why:",
                "\n - I don't have access to the channel the message is in.",
                "\n - The message doesn't exist.",
                "\n - The message doesn't have any upvotes, so it isn't in the database.",
            ), true).await?;
            return Ok(());
        };

        let link = fmt_message_link(guild_id, sql_msg.channel_id, sql_msg.message_id);
        let mut emb = embed::build().description(concat_format!(
            "original: `{}` [jump]({})" <- sql_msg.message_id, link;
            "\nchannel: `{0}` <#{0}>" <- sql_msg.channel_id;
            "\nauthor: `{0}` <@{0}>" <- sql_msg.author_id;
            "\n\ntrashed: {:?}" <- sql_msg.trashed;
            "\nfrozen: {:?}" <- sql_msg.frozen;
        ));

        for starboard in Starboard::list_by_guild(&ctx.bot.pool, guild_id).await? {
            let sb_msg =
                StarboardMessage::get_by_starboard(&ctx.bot.pool, sql_msg.message_id, starboard.id)
                    .await?;
            let link = sb_msg
                .map(|m| fmt_message_link(guild_id, starboard.channel_id, m.message_id))
                .map(|link| format!("[jump]({link})"))
                .unwrap_or_else(|| "Not on starboard.".to_string());
            emb = emb.field(
                EmbedFieldBuilder::new(
                    starboard.name,
                    concat_format!(
                        "{}\n" <- link;
                        "forced: {}" <- sql_msg.forced_to.contains(&starboard.id);
                    ),
                )
                .build(),
            );
        }

        ctx.respond(ctx.build_resp().embeds([emb.build()]).build())
            .await?;

        Ok(())
    }
}
