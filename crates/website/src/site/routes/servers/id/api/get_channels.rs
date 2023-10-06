use leptos::*;
use twilight_model::{
    channel::Channel,
    id::{marker::GuildMarker, Id},
};

#[server(GetChannels, "/api")]
pub async fn get_channels(
    guild_id: Id<GuildMarker>,
) -> Result<(Vec<Channel>, Vec<Channel>), ServerFnError> {
    use crate::site::routes::servers::id::api::can_manage_guild;

    can_manage_guild(guild_id).await?;

    let http = crate::bot_http();
    let channels = http.guild_channels(guild_id).await?.model().await?;

    let active_threads = http.active_threads(guild_id).await?.model().await?;

    Ok((channels, active_threads.threads))
}
