use twilight_model::application::{
    command::CommandOptionChoice, interaction::ApplicationCommandAutocomplete,
};

use crate::{client::bot::StarboardBot, database::AutoStarChannel, unwrap_id};

pub async fn autostar_name_autocomplete(
    bot: &StarboardBot,
    interaction: &Box<ApplicationCommandAutocomplete>,
) -> anyhow::Result<Vec<CommandOptionChoice>> {
    let names: Vec<String> = match bot
        .cache
        .guild_autostar_channel_names
        .get(&interaction.guild_id.unwrap())
    {
        Some(names) => (**names.value()).clone(),
        None => {
            AutoStarChannel::list_by_guild(&bot.pool, unwrap_id!(interaction.guild_id.unwrap()))
                .await?
                .into_iter()
                .map(|a| a.name)
                .collect()
        }
    };

    let mut arr = Vec::new();
    for name in names.into_iter() {
        arr.push(CommandOptionChoice::String {
            name: name.clone(),
            name_localizations: None,
            value: name,
        })
    }

    Ok(arr)
}
