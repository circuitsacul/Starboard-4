use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{channel::message::Embed, guild::Role};

use crate::{
    client::bot::StarboardBot,
    concat_format,
    database::{models::permrole::SortVecPermRole, PermRole, PermRoleStarboard, Starboard},
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    locale_func,
    translations::Lang,
    utils::{
        embed,
        id_as_i64::GetI64,
        into_id::IntoId,
        views::select_paginator::{SelectPaginatorBuilder, SelectPaginatorPageBuilder},
    },
};

// todo: formatter
macro_rules! fmt_trib {
    ($to_fmt: expr) => {
        $to_fmt
            .map(|v| v.to_string())
            .unwrap_or("default".to_string())
    };
}

locale_func!(permroles_view);
locale_func!(permroles_view_option_role);

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "view",
    desc = "View the PermRoles for this server.",
    desc_localizations = "permroles_view"
)]
pub struct ViewPermRoles {
    /// The PermRole to view settings for.
    #[command(desc_localizations = "permroles_view_option_role")]
    role: Option<Role>,
}

impl ViewPermRoles {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx);
        let bot = ctx.bot.clone();
        let lang = ctx.user_lang();

        let mut perm_roles = PermRole::list_by_guild(&ctx.bot.pool, guild_id.get_i64()).await?;

        if perm_roles.is_empty() {
            ctx.respond_str(lang.permroles_view_none(), true).await?;
            return Ok(());
        }

        let mut paginator = SelectPaginatorBuilder::new(ctx);
        let mut current = 0;

        perm_roles.sort_permroles(&bot);
        for (idx, pr) in perm_roles.into_iter().rev().enumerate() {
            if self.role.as_ref().map(|r| r.id.get_i64()) == Some(pr.role_id) {
                current = idx;
            }

            let name = bot.cache.guilds.with(&guild_id, |_, guild| {
                let Some(guild) = guild else {
                    return None;
                };

                guild
                    .roles
                    .get(&pr.role_id.into_id())
                    .map(|r| r.name.to_owned())
            });
            let label = name.unwrap_or_else(|| lang.deleted_role(pr.role_id));
            let embed = permrole_embed(&bot, pr, lang).await?;

            let page = SelectPaginatorPageBuilder::new(label).add_embed(embed);
            paginator = paginator.add_page(page);
        }

        paginator.current(current).build().run().await?;

        Ok(())
    }
}

async fn permrole_embed(bot: &StarboardBot, pr: PermRole, lang: Lang) -> StarboardResult<Embed> {
    let mut pr_config = lang.permroles_view_title(pr.role_id);
    pr_config.push_str(&concat_format!(
        "vote: {}\n" <- fmt_trib!(pr.give_votes);
        "receive-votes: {}\n" <- fmt_trib!(pr.receive_votes);
        "xproles: {}\n" <- fmt_trib!(pr.obtain_xproles);
    ));

    let permrole_sbs = PermRoleStarboard::list_by_permrole(&bot.pool, pr.role_id).await?;

    for pr_sb in permrole_sbs {
        let sb = Starboard::get(&bot.pool, pr_sb.starboard_id).await?;
        let sb = match sb {
            None => {
                eprintln!("Starboard for PermRole didn't exist. This shouldn't happen.");
                continue;
            }
            Some(sb) => sb,
        };

        pr_config.push_str(&lang.permroles_view_sb_title(sb.channel_id, sb.name));
        pr_config.push_str(&concat_format!(
            "vote: {}\n" <- fmt_trib!(pr_sb.give_votes);
            "receive-votes: {}\n" <- fmt_trib!(pr_sb.receive_votes);
        ));
    }

    let embed = embed::build()
        .title(lang.permroles_title())
        .description(pr_config)
        .build();

    Ok(embed)
}
