use twilight_model::application::interaction::MessageComponentInteraction;

use crate::client::bot::StarboardBot;

pub async fn handle_component(
    bot: StarboardBot,
    interaction: Box<MessageComponentInteraction>,
) -> anyhow::Result<()> {
    match interaction.data.custom_id.as_str() {
        _ => {}
    }

    Ok(())
}
