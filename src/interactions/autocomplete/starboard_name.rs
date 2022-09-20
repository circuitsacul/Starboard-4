use twilight_model::application::command::CommandOptionChoice;

use crate::{database::Starboard, interactions::context::CommandCtx, unwrap_id};

pub async fn starboard_name_autocomplete(
    ctx: &CommandCtx,
) -> anyhow::Result<Vec<CommandOptionChoice>> {
    let guild_id = ctx.interaction.guild_id.unwrap();
    let names: Vec<String> = match ctx.bot.cache.guild_autostar_channel_names.get(&guild_id) {
        Some(names) => (*names.value()).clone(),
        None => Starboard::list_by_guild(&ctx.bot.pool, unwrap_id!(guild_id))
            .await?
            .into_iter()
            .map(|a| a.name)
            .collect(),
    };

    let mut arr = Vec::new();
    for name in names {
        arr.push(CommandOptionChoice::String {
            name: name.clone(),
            name_localizations: None,
            value: name,
        });
    }

    Ok(arr)
}
