use twilight_model::application::command::{CommandOptionChoice, CommandOptionChoiceData};

use crate::{
    database::DbMember,
    errors::StarboardResult,
    interactions::context::CommandCtx,
    utils::{id_as_i64::GetI64, into_id::IntoId},
};

pub async fn autoredeem_autocomplete(
    ctx: &CommandCtx,
    focused: &str,
) -> StarboardResult<Vec<CommandOptionChoice>> {
    let user_id = ctx.interaction.author_id().unwrap().get_i64();

    let guild_ids = DbMember::list_autoredeem_by_user(&ctx.bot.pool, user_id).await?;

    let mut arr = Vec::new();
    for guild_id in guild_ids {
        let name = ctx.bot.cache.guilds.with(&guild_id.into_id(), |_, guild| {
            if let Some(guild) = &guild {
                if !focused.is_empty()
                    && !guild
                        .name
                        .to_lowercase()
                        .starts_with(&focused.to_lowercase())
                {
                    None
                } else {
                    Some(guild.name.clone())
                }
            } else {
                Some(format!("Deleted Guild {guild_id}"))
            }
        });
        let Some(name) = name else { continue; };
        arr.push(CommandOptionChoice::String(CommandOptionChoiceData {
            name: name.clone(),
            name_localizations: None,
            value: guild_id.to_string(),
        }));
    }

    Ok(arr)
}
