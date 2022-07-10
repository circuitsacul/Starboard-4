use std::sync::Arc;

use twilight_model::application::interaction::MessageComponentInteraction;

use crate::client::bot::StarboardBot;

pub async fn handle_dismiss(
    bot: Arc<StarboardBot>,
    interaction: Box<MessageComponentInteraction>,
) -> anyhow::Result<()> {
    assert!(interaction.is_dm());

    bot.http
        .delete_message(interaction.message.channel_id, interaction.message.id)
        .exec()
        .await?;

    Ok(())
}
