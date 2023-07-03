use std::borrow::Cow;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::channel::message::MessageFlags;
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{
    constants,
    database::{DbGuild, DbMember, DbUser},
    errors::StarboardResult,
    interactions::context::CommandCtx,
    locale_func,
    utils::{embed, id_as_i64::GetI64, into_id::IntoId},
};

locale_func!(premium_info);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "info",
    desc = "Get premium info.",
    desc_localizations = "premium_info"
)]
pub struct Info;

impl Info {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let user_id = ctx.interaction.author_id().unwrap().get_i64();
        let lang = ctx.user_lang();

        let user = DbUser::get(&ctx.bot.pool, user_id).await?;
        let credits = match &user {
            Some(user) => user.credits,
            None => 0,
        };

        let mut emb = embed::build()
            .title(lang.premium_emb_title())
            .description(lang.premium_emb_desc(constants::PATREON_URL))
            .field(EmbedFieldBuilder::new(
                lang.premium_emb_status(),
                lang.premium_emb_status_value(credits),
            ));

        'out: {
            if user.is_none() {
                break 'out;
            }

            let ar = DbMember::list_autoredeem_by_user(&ctx.bot.pool, user_id).await?;
            if ar.is_empty() {
                break 'out;
            }

            let mut value = lang.premium_emb_ar_top().to_string();
            for guild_id in ar {
                ctx.bot.cache.guilds.with(&guild_id.into_id(), |_, guild| {
                    if let Some(guild) = &guild {
                        value.push_str(&guild.name);
                        value.push('\n');
                    } else {
                        value.push_str(&lang.unknown_server(guild_id));
                    }
                });
            }

            value.push_str(lang.premium_emb_ar_desc());

            emb = emb.field(EmbedFieldBuilder::new(lang.premium_emb_ar(), value));
        }

        if let Some(guild_id) = ctx.interaction.guild_id {
            let guild = DbGuild::get(&ctx.bot.pool, guild_id.get_i64()).await?;

            let value = match guild.and_then(|g| g.premium_end) {
                None => Cow::Borrowed(lang.premium_end_no_premium()),
                Some(end) => Cow::Owned(lang.premium_end_premium_until(end.timestamp())),
            };
            emb = emb.field(EmbedFieldBuilder::new(lang.premium_emb_server(), value));
        };

        ctx.respond(
            ctx.build_resp()
                .embeds([emb.build()])
                .flags(MessageFlags::EPHEMERAL)
                .build(),
        )
        .await?;

        Ok(())
    }
}
