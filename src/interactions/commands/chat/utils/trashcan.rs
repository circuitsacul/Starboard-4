use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::Message,
    errors::StarboardResult,
    get_guild_id,
    interactions::context::CommandCtx,
    utils::{id_as_i64::GetI64, message_link::fmt_message_link, views::paginator},
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "trashcan", desc = "View all trashed messages.")]
pub struct TrashCan;

impl TrashCan {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let guild_id = get_guild_id!(ctx).get_i64();

        let trashed = Message::list_trashed(&ctx.bot.pool, guild_id).await?;

        if trashed.is_empty() {
            ctx.respond_str("There are no trashed messages.", true)
                .await?;
            return Ok(());
        }

        let mut pages = Vec::new();
        let mut curr_page = String::new();

        for (idx, item) in trashed.into_iter().enumerate() {
            if idx % 50 == 0 && idx != 0 {
                pages.push(curr_page);
                curr_page = String::new();
            }

            let link = fmt_message_link(guild_id, item.channel_id, item.message_id);
            curr_page.push_str(&format!(
                "[{}]({})\n",
                item.trash_reason
                    .unwrap_or_else(|| "No reason given.".to_string()),
                link,
            ));
        }

        pages.push(curr_page);

        let author_id = ctx.interaction.author_id().unwrap();
        paginator::simple(
            &mut ctx,
            pages.into_iter().map(|page| (Some(page), None)).collect(),
            author_id,
            true,
        )
        .await?;

        Ok(())
    }
}
