use twilight_model::application::interaction::MessageComponentInteraction;

use crate::client::bot::StarboardBot;

use super::dismiss::handle_dismiss;

pub async fn handle_component(
    bot: StarboardBot,
    interaction: Box<MessageComponentInteraction>,
) -> anyhow::Result<()> {
    match interaction.data.custom_id.as_str() {
        "stateless::dismiss_notification" => handle_dismiss(bot, interaction).await?,
        _ => {}
    }

    Ok(())
}
