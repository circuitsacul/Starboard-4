use twilight_model::id::{marker::GuildMarker, Id};

use crate::{
    client::bot::StarboardBot,
    database::{models::permrole::SortVecPermRole, PermRole, PermRoleStarboard},
    errors::StarboardResult,
    unwrap_id,
};

pub struct Permissions {
    give_votes: bool,
    receive_votes: bool,
    obtain_xproles: bool,
}

impl Default for Permissions {
    fn default() -> Self {
        Self::new()
    }
}

impl Permissions {
    pub fn new() -> Self {
        Self {
            give_votes: true,
            receive_votes: true,
            obtain_xproles: true,
        }
    }

    pub async fn get_permissions(
        bot: &StarboardBot,
        roles: &[i64],
        guild_id: Id<GuildMarker>,
        starboard_id: Option<i32>,
    ) -> StarboardResult<Self> {
        let mut perms = Self::new();

        // get permroles
        let permroles = PermRole::list_by_guild(&bot.pool, unwrap_id!(guild_id)).await?;
        // filter out non-applicable permroles
        let mut permroles = permroles
            .into_iter()
            .filter(|r| roles.contains(&r.role_id))
            .collect::<Vec<_>>();
        // sort permroles by their order in the guild
        permroles.sort_permroles(bot);

        for pr in permroles {
            if let Some(val) = pr.give_votes {
                perms.give_votes = val;
            }
            if let Some(val) = pr.receive_votes {
                perms.receive_votes = val;
            }
            if let Some(val) = pr.obtain_xproles {
                perms.obtain_xproles = val;
            }

            if let Some(sb_id) = starboard_id {
                let pr_sb = PermRoleStarboard::get(&bot.pool, pr.role_id, sb_id).await?;

                if let Some(val) = pr_sb.give_votes {
                    perms.give_votes = val;
                }
                if let Some(val) = pr_sb.receive_votes {
                    perms.receive_votes = val;
                }
            }
        }

        Ok(perms)
    }
}
