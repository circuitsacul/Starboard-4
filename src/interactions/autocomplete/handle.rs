use twilight_model::{
    application::interaction::application_command::{CommandDataOption, CommandOptionValue},
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{errors::StarboardResult, interactions::context::CommandCtx};

use super::{
    autostar_name::autostar_name_autocomplete, override_name::override_name_autocomplete,
    starboard_name::starboard_name_autocomplete,
};

pub fn get_sub_options(options: &Vec<CommandDataOption>) -> Option<&Vec<CommandDataOption>> {
    if options.len() != 1 {
        return None;
    }

    match &options[0].value {
        CommandOptionValue::SubCommand(options) => Some(options),
        CommandOptionValue::SubCommandGroup(options) => Some(options),
        _ => None,
    }
}

pub fn qualified_name(ctx: &CommandCtx) -> String {
    let mut name = ctx.data.name.clone();
    let options;

    if let Some(sub_options) = get_sub_options(&ctx.data.options) {
        let sub = &ctx.data.options[0];
        name.push(' ');
        name.push_str(&sub.name);
        if let Some(subsub_options) = get_sub_options(sub_options) {
            let subcommand = &sub_options[0];
            name.push(' ');
            name.push_str(&subcommand.name);
            options = subsub_options;
        } else {
            options = sub_options;
        }
    } else {
        options = &ctx.data.options;
    }

    for option in options {
        if matches!(option.value, CommandOptionValue::Focused(_, _)) {
            name.push(' ');
            name.push_str(&option.name);
            return name;
        }
    }

    unreachable!("No focused option in autocomplete response.");
}

pub async fn handle_autocomplete(ctx: CommandCtx) -> StarboardResult<()> {
    let options = match qualified_name(&ctx).as_str() {
        // misc
        "random starboard" => starboard_name_autocomplete(&ctx).await?,
        "utils force starboard" => starboard_name_autocomplete(&ctx).await?,
        // autostar channels
        "autostar delete name" => autostar_name_autocomplete(&ctx).await?,
        "autostar view name" => autostar_name_autocomplete(&ctx).await?,
        "autostar edit name" => autostar_name_autocomplete(&ctx).await?,
        "autostar rename current-name" => autostar_name_autocomplete(&ctx).await?,
        // starboards
        "starboards delete name" => starboard_name_autocomplete(&ctx).await?,
        "starboards view name" => starboard_name_autocomplete(&ctx).await?,
        "starboards edit embed name" => starboard_name_autocomplete(&ctx).await?,
        "starboards edit style name" => starboard_name_autocomplete(&ctx).await?,
        "starboards edit requirements name" => starboard_name_autocomplete(&ctx).await?,
        "starboards edit behavior name" => starboard_name_autocomplete(&ctx).await?,
        "starboards rename current-name" => starboard_name_autocomplete(&ctx).await?,
        // overrides
        "overrides create starboard" => starboard_name_autocomplete(&ctx).await?,
        "overrides delete name" => override_name_autocomplete(&ctx).await?,
        "overrides rename current-name" => override_name_autocomplete(&ctx).await?,
        "overrides channels set override" => override_name_autocomplete(&ctx).await?,
        "overrides channels remove override" => override_name_autocomplete(&ctx).await?,
        "overrides channels add override" => override_name_autocomplete(&ctx).await?,
        "overrides edit requirements name" => override_name_autocomplete(&ctx).await?,
        "overrides edit behavior name" => override_name_autocomplete(&ctx).await?,
        "overrides edit style name" => override_name_autocomplete(&ctx).await?,
        "overrides edit embed-style name" => override_name_autocomplete(&ctx).await?,
        "overrides edit reset name" => override_name_autocomplete(&ctx).await?,
        "overrides view name" => override_name_autocomplete(&ctx).await?,
        // permroles
        "permroles edit-starboard starboard" => starboard_name_autocomplete(&ctx).await?,
        qual => todo!("Unexpected autocomplete for {}.", qual),
    };

    let i = ctx.bot.interaction_client().await;
    let data = InteractionResponseDataBuilder::new()
        .choices(options)
        .build();
    let resp = InteractionResponse {
        data: Some(data),
        kind: InteractionResponseType::ApplicationCommandAutocompleteResult,
    };
    i.create_response(ctx.interaction.id, &ctx.interaction.token, &resp)
        .await?;

    Ok(())
}
