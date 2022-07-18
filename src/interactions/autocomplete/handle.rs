use std::sync::Arc;

use twilight_model::{
    application::interaction::{
        application_command_autocomplete::{
            ApplicationCommandAutocompleteDataOption, ApplicationCommandAutocompleteDataOptionType,
        },
        ApplicationCommandAutocomplete,
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::client::bot::StarboardBot;

use super::{
    autostar_name::autostar_name_autocomplete, starboard_name::starboard_name_autocomplete,
};

pub fn option_is_sub(options: &Vec<ApplicationCommandAutocompleteDataOption>) -> bool {
    if options.len() != 1 {
        return false;
    }

    {
        matches!(
            options[0].kind,
            ApplicationCommandAutocompleteDataOptionType::SubCommand
        ) || matches!(
            options[0].kind,
            ApplicationCommandAutocompleteDataOptionType::SubCommandGroup
        )
    }
}

pub fn qualified_name(interaction: &Box<ApplicationCommandAutocomplete>) -> String {
    let mut name = interaction.data.name.clone();
    let options: Option<_>;

    if option_is_sub(&interaction.data.options) {
        let sub = &interaction.data.options[0];
        name.push_str(" ");
        name.push_str(&sub.name);
        if option_is_sub(&sub.options) {
            let subcommand = &sub.options[0];
            name.push_str(" ");
            name.push_str(&subcommand.name);
            options = Some(&subcommand.options);
        } else {
            options = Some(&sub.options);
        }
    } else {
        options = Some(&interaction.data.options);
    }

    for option in options.unwrap() {
        if option.focused {
            name.push_str(" ");
            name.push_str(&option.name);
            return name;
        }
    }

    panic!("No focused option in autocomplete response.");
}

pub async fn handle_autocomplete(
    bot: Arc<StarboardBot>,
    interaction: Box<ApplicationCommandAutocomplete>,
) -> anyhow::Result<()> {
    let options = match qualified_name(&interaction).as_str() {
        // autostar channels
        "autostar delete name" => autostar_name_autocomplete(&bot, &interaction).await?,
        "autostar view name" => autostar_name_autocomplete(&bot, &interaction).await?,
        "autostar edit name" => autostar_name_autocomplete(&bot, &interaction).await?,
        "autostar rename current-name" => autostar_name_autocomplete(&bot, &interaction).await?,
        // starboards
        "starboards delete name" => starboard_name_autocomplete(&bot, &interaction).await?,
        "starboards view name" => starboard_name_autocomplete(&bot, &interaction).await?,
        "starboards edit embed name" => starboard_name_autocomplete(&bot, &interaction).await?,
        "starboards edit style name" => starboard_name_autocomplete(&bot, &interaction).await?,
        "starboards edit requirements name" => {
            starboard_name_autocomplete(&bot, &interaction).await?
        }
        qual => todo!("Unexpected autocomplete for {}.", qual),
    };

    let i = bot.interaction_client().await;
    let data = InteractionResponseDataBuilder::new()
        .choices(options)
        .build();
    let resp = InteractionResponse {
        data: Some(data),
        kind: InteractionResponseType::ApplicationCommandAutocompleteResult,
    };
    i.create_response(interaction.id, &interaction.token, &resp)
        .exec()
        .await?;

    Ok(())
}
