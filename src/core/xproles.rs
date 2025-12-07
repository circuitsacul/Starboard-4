use twilight_model::id::{
    Id,
    marker::{GuildMarker, UserMarker},
};

use crate::{
    client::bot::StarboardBot,
    database::{DbMember, XPRole},
    errors::StarboardResult,
    utils::{id_as_i64::GetI64, into_id::IntoId},
};

pub async fn refresh_xpr(
    bot: &StarboardBot,
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
) -> StarboardResult<()> {
    let Some(member) = bot.cache.fog_member(bot, guild_id, user_id).await? else {
        return Ok(());
    };
    let member_roles = &member.roles;

    let xproles = XPRole::list_by_guild(&bot.pool, guild_id.get_i64()).await?;
    let Some(member) = DbMember::get(&bot.pool, guild_id.get_i64(), user_id.get_i64()).await?
    else {
        return Ok(());
    };

    for xpr in xproles {
        let role_id = xpr.role_id.into_id();
        if member.xp >= xpr.required as f32 {
            if member_roles.contains(&role_id) {
                continue;
            }

            let _ = bot
                .http
                .add_guild_member_role(guild_id, user_id, role_id)
                .await;
        } else {
            if !member_roles.contains(&role_id) {
                continue;
            }

            let _ = bot
                .http
                .remove_guild_member_role(guild_id, user_id, role_id)
                .await;
        }
    }

    Ok(())
}
