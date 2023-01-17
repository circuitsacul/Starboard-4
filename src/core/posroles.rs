use std::sync::Arc;

use twilight_model::id::{
    marker::{GuildMarker, RoleMarker, UserMarker},
    Id,
};

use crate::{
    client::bot::StarboardBot,
    constants,
    database::{Member, PosRole},
    errors::StarboardResult,
    utils::{id_as_i64::GetI64, into_id::IntoId},
};

use super::premium::is_premium::is_guild_premium;

pub struct GuildPRUpdateResult {
    pub removed_roles: i32,
    pub added_roles: i32,
    pub failed_removals: i32,
    pub failed_adds: i32,
}

pub async fn loop_update_posroles(bot: Arc<StarboardBot>) {
    loop {
        tokio::time::sleep(constants::UPDATE_PRS_DELAY).await;

        let guilds = sqlx::query!("SELECT DISTINCT guild_id FROM posroles")
            .fetch_all(&bot.pool)
            .await;

        let guilds = match guilds {
            Ok(guilds) => guilds,
            Err(err) => {
                bot.handle_error(&err.into()).await;
                continue;
            }
        };

        let mut tasks = Vec::new();
        for guild in guilds {
            let is_prem = match is_guild_premium(&bot, guild.guild_id).await {
                Ok(is_prem) => is_prem,
                Err(why) => {
                    bot.handle_error(&why).await;
                    continue;
                }
            };
            if !is_prem {
                continue;
            }
            let task = tokio::spawn(update_posroles_for_guild(bot.clone(), guild.guild_id.into_id()));
            tasks.push(task);
        }

        for t in tasks {
            let ret = t.await;
            let ret = match ret {
                Ok(ret) => ret,
                Err(err) => {
                    bot.handle_error(&err.into()).await;
                    continue;
                }
            };
            if let Err(err) = ret {
                bot.handle_error(&err.into()).await;
            }
        }
    }
}

pub async fn update_posroles_for_guild(
    bot: Arc<StarboardBot>,
    guild_id: Id<GuildMarker>,
) -> StarboardResult<Option<GuildPRUpdateResult>> {
    let Some(_lock) = bot.locks.guild_pr_update.lock(guild_id.get_i64()) else {
        return Ok(None);
    };

    let mut removed_roles = 0;
    let mut added_roles = 0;
    let mut failed_removals = 0;
    let mut failed_adds = 0;

    let guild_id_i64 = guild_id.get_i64();

    let posroles = PosRole::list_by_guild(&bot.pool, guild_id_i64).await?;
    let pr_ids: Vec<Id<RoleMarker>> = posroles.iter().map(|pr| pr.role_id.into_id()).collect();

    let lb_size: i32 = posroles.iter().map(|pr| pr.max_members).sum();
    let mut leaderboard = Member::list_by_xp_exclude_deleted(
        &bot.pool,
        guild_id.get_i64(),
        lb_size as i64,
        &bot.cache,
    )
    .await?;

    for pr in posroles {
        let role_id: Id<RoleMarker> = pr.role_id.into_id();

        let to_drain = [pr.max_members as usize, leaderboard.len()]
            .into_iter()
            .min()
            .unwrap();
        for member in leaderboard.drain(..to_drain) {
            let user_id: Id<UserMarker> = member.user_id.into_id();

            let Some((to_remove, has_current)) = bot.cache.guilds.with(&guild_id, |_, g| {
                let Some(g) = g else {
                    return None;
                };
                let Some(member) = g.members.get(&user_id) else {
                    return None;
                };

                let to_remove: Vec<_> = pr_ids.iter().filter(|pr| member.roles.contains(*pr) && **pr != role_id).collect();
                let has_current = member.roles.contains(&role_id);

                Some((to_remove, has_current))
            }) else {
                continue;
            };

            if !has_current {
                let ret = bot
                    .http
                    .add_guild_member_role(guild_id, user_id, role_id)
                    .await;
                if ret.is_err() {
                    failed_adds += 1;
                } else {
                    added_roles += 1;
                }
            }

            for role in to_remove {
                let ret = bot
                    .http
                    .remove_guild_member_role(guild_id, user_id, *role)
                    .await;
                if ret.is_err() {
                    failed_removals += 1;
                } else {
                    removed_roles += 1;
                }
            }
        }
    }

    Ok(Some(GuildPRUpdateResult {
        added_roles,
        removed_roles,
        failed_adds,
        failed_removals,
    }))
}
