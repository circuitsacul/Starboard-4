use twilight_model::application::interaction::Interaction;

use crate::client::bot::StarboardBot;
use crate::interactions::commands::handle::handle_command;

use super::autocomplete::handle::handle_autocomplete;

pub async fn handle_interaction(
    shard_id: u64,
    interaction: Interaction,
    bot: StarboardBot,
) -> anyhow::Result<()> {
    match interaction {
        Interaction::ApplicationCommand(interaction) => {
            handle_command(shard_id, bot, interaction).await?
        }
        Interaction::ApplicationCommandAutocomplete(interaction) => {
            handle_autocomplete(bot, interaction).await?
        }
        _ => {}
    }

    Ok(())
}
