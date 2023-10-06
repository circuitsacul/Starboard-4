use leptos::*;
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker},
    Id,
};

use crate::site::components::form::ValidationErrors;

/// TODO: validate channel existence and type
#[server(CreateStarboard, "/api")]
pub async fn create_starboard(
    guild_id: Id<GuildMarker>,
    channel_id: Option<Id<ChannelMarker>>,
    name: String,
) -> Result<ValidationErrors, ServerFnError> {
    use database::{validation::name::validate_name, Starboard};
    use errors::ErrToStr;
    use leptos_actix::redirect;

    use crate::site::routes::servers::id::api::can_manage_guild;

    let mut errors = ValidationErrors::new();

    let Some(channel_id) = channel_id else {
        errors.insert("channel_id".into(), "Please select a channel.".into());
        return Ok(errors);
    };

    can_manage_guild(guild_id).await?;

    let db = crate::db();

    let name = match validate_name(&name) {
        Ok(name) => name,
        Err(why) => {
            errors.insert("name".into(), why.to_web_str());
            return Ok(errors);
        }
    };

    let sb = Starboard::create(&db, &name, channel_id.get() as _, guild_id.get() as _).await?;
    let Some(sb) = sb else {
        errors.insert("name".into(), "That name is already in use.".into());
        return Ok(errors);
    };

    redirect(&format!(
        "/servers/{}/starboards/{}",
        guild_id,
        &sb.id.to_string()
    ));

    Ok(errors)
}
