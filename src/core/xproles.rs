use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

use crate::{
    client::bot::StarboardBot,
    database::{Member, XPRole},
    errors::StarboardResult,
    utils::{id_as_i64::GetI64, into_id::IntoId},
};

pub async fn refresh_xpr(
    bot: &StarboardBot,
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
) -> StarboardResult<()> {
    let member_roles = bot.cache.guilds.with(&guild_id, |_, g| {
        let Some(g) = g else {
            return None;
        };
        g.members.get(&user_id).map(|m| m.roles.to_owned())
    });
    let Some(member_roles) = member_roles else {
        return Ok(());
    };

    let xproles = XPRole::list_by_guild(&bot.pool, guild_id.get_i64()).await?;
    let Some(member) = Member::get(&bot.pool, guild_id.get_i64(), user_id.get_i64()).await? else {
        return Ok(());
    };

    for xpr in xproles {
        let role_id = xpr.role_id.into_id();
        if member.xp >= xpr.required as f32 {
            if member_roles.contains(&role_id) {
                continue;
            }

            bot.http
                .add_guild_member_role(guild_id, user_id, role_id)
                .await?;
        } else {
            if !member_roles.contains(&role_id) {
                continue;
            }

            bot.http
                .remove_guild_member_role(guild_id, user_id, role_id)
                .await?;
        }
    }

    Ok(())
}
