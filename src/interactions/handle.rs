use std::sync::Arc;

use twilight_model::application::interaction::{Interaction, InteractionData, InteractionType};

use crate::{client::bot::StarboardBot, errors::StarboardResult};

use super::{
    autocomplete::handle::handle_autocomplete, commands::handle::handle_command,
    components::handle::handle_component, context::Ctx,
};

pub async fn handle_interaction(
    shard_id: u64,
    interaction: Interaction,
    bot: Arc<StarboardBot>,
) -> StarboardResult<()> {
    match interaction.data {
        Some(InteractionData::ApplicationCommand(ref data)) => {
            let data = data.to_owned();
            let ctx = Ctx::new(shard_id, bot, interaction, data);
            if matches!(
                ctx.interaction.kind,
                InteractionType::ApplicationCommandAutocomplete
            ) {
                handle_autocomplete(ctx).await?;
            } else {
                handle_command(ctx).await?;
            }
        }
        Some(InteractionData::MessageComponent(ref data)) => {
            let data = data.to_owned();
            let ctx = Ctx::new(shard_id, bot, interaction, data);
            handle_component(ctx).await?;
        }
        _ => {}
    }

    Ok(())
}
