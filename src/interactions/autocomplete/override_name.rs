use twilight_model::application::command::{CommandOptionChoice, CommandOptionChoiceData};

use crate::{
    database::StarboardOverride, errors::StarboardResult, interactions::context::CommandCtx,
    utils::id_as_i64::GetI64,
};

pub async fn override_name_autocomplete(
    ctx: &CommandCtx,
) -> StarboardResult<Vec<CommandOptionChoice>> {
    let guild_id = ctx.interaction.guild_id.unwrap();
    let names: Vec<String> = match ctx.bot.cache.guild_override_names.get(&guild_id) {
        Some(names) => (*names.value()).clone(),
        None => StarboardOverride::list_by_guild(&ctx.bot.pool, guild_id.get_i64())
            .await?
            .into_iter()
            .map(|v| v.name)
            .collect(),
    };

    let mut arr = Vec::new();
    for name in names {
        arr.push(CommandOptionChoice::String(CommandOptionChoiceData {
            name: name.clone(),
            name_localizations: None,
            value: name,
        }));
    }

    Ok(arr)
}
