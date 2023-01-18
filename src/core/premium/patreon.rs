use std::{collections::HashMap, sync::Arc};

use crate::{
    client::bot::StarboardBot,
    constants,
    database::{Patron, User},
    errors::StarboardResult,
    map_dup_none,
    utils::into_id::IntoId,
};

use super::roles::update_supporter_roles;

#[derive(Debug)]
struct PatronData {
    pub patreon_id: String,
    pub total_cents: u64,
    pub discord_id: Option<i64>,
    /// 0=none, 1=active, 2=declined, 3=former
    pub status: i16,
}

pub async fn patreon_loop(bot: Arc<StarboardBot>) {
    if bot.config.patreon_token.is_none() {
        return eprintln!("Warning: no Patreon token set.");
    }

    loop {
        tokio::time::sleep(constants::UPDATE_PATREON_DELAY).await;

        let task = tokio::spawn(StarboardBot::catch_future_errors(
            bot.clone(),
            update_patrons(bot.clone()),
        ));

        if let Err(err) = task.await {
            bot.handle_error(&err.into()).await;
        };
    }
}

pub async fn update_patrons(bot: Arc<StarboardBot>) -> StarboardResult<()> {
    let token = bot.config.patreon_token.as_ref().unwrap();

    let patrons = get_patrons_from_patreon(&bot, token).await?;

    for patron in patrons {
        let sql_patron = map_dup_none!(Patron::create(&bot.pool, &patron.patreon_id))?;
        let mut sql_patron = match sql_patron {
            Some(sql_patron) => sql_patron,
            None => Patron::get(&bot.pool, &patron.patreon_id).await?.unwrap(),
        };

        // update the discord ID if needed
        if sql_patron.discord_id != patron.discord_id {
            if let Some(old_user_id) = sql_patron.discord_id {
                // moved or unlinked discord account
                User::set_patreon_status(&bot.pool, old_user_id, 0).await?;

                let clone = bot.clone();
                tokio::spawn(async move {
                    let ret = update_supporter_roles(&clone, old_user_id.into_id()).await;
                    if let Err(err) = ret {
                        clone.handle_error(&err).await;
                    }
                });
                // TODO: notify them
            }

            sql_patron.discord_id = patron.discord_id;

            if let Some(user_id) = patron.discord_id {
                map_dup_none!(User::create(&bot.pool, user_id, false))?;
            }
            Patron::set_discord_id(&bot.pool, &patron.patreon_id, patron.discord_id).await?;
        }

        // add credits the corresponding user, if needed
        let Some(user_id) = patron.discord_id else { continue; };
        let user = User::get(&bot.pool, user_id).await?.unwrap();

        let cents_difference = patron.total_cents as i64 - sql_patron.last_patreon_total_cents;
        if cents_difference > 0 {
            let credits = (cents_difference as f64 / 100_f64).round() as i32;
            User::add_credits(&bot.pool, user.user_id, credits).await?;
            Patron::set_total_cents(&bot.pool, &patron.patreon_id, patron.total_cents as i64)
                .await?;
        }

        // update the patron status
        if user.patreon_status != patron.status {
            User::set_patreon_status(&bot.pool, user.user_id, patron.status).await?;
            // TODO: notify user for status change
        }
    }

    Ok(())
}

async fn get_patrons_from_patreon(
    bot: &StarboardBot,
    token: &str,
) -> StarboardResult<Vec<PatronData>> {
    let campaign = fetch(
        bot,
        "https://www.patreon.com/api/oauth2/v2/campaigns",
        token,
    )
    .await?;

    let campid = campaign["data"][0]["id"].as_str().unwrap();

    let mut next_url: Option<String> = None;
    let mut patrons: Vec<PatronData> = Vec::new();

    loop {
        let page = if let Some(url) = &next_url {
            fetch(bot, url, token).await?
        } else {
            let first_url = format!(
                concat!(
                    "https://www.patreon.com/api/oauth2/v2/campaigns/{}/",
                    "members?fields%5Bmember%5D=campaign_lifetime_support_cents,",
                    "patron_status&include=user&fields%5Buser%5D=social_connections"
                ),
                campid
            );
            fetch(bot, &first_url, token).await?
        };

        let mut discord_ids: HashMap<&str, Option<i64>> = HashMap::new();
        for user in page["included"].as_array().unwrap() {
            let discord_id = user["attributes"]["social_connections"]["discord"]["user_id"]
                .as_str()
                .map(|id| id.parse().unwrap());
            discord_ids.insert(user["id"].as_str().unwrap(), discord_id);
        }

        for patron in page["data"].as_array().unwrap() {
            let user_id = patron["relationships"]["user"]["data"]["id"]
                .as_str()
                .unwrap();
            let discord_id = discord_ids[user_id];

            let Some(str_status) = patron["attributes"]["patron_status"].as_str() else {
                continue;
            };
            let status: i16 = match str_status {
                "active_patron" => 1,
                "declined_patron" => 2,
                "former_patron" => 3,
                _ => 0,
            };

            let patron = PatronData {
                patreon_id: patron["id"].as_str().unwrap().to_owned(),
                discord_id,
                total_cents: patron["attributes"]["campaign_lifetime_support_cents"]
                    .as_u64()
                    .unwrap(),
                status,
            };
            patrons.push(patron);
        }

        next_url = page["links"]["next"].as_str().map(|v| v.to_string());
        if next_url.is_none() {
            break;
        }
    }

    Ok(patrons)
}

async fn fetch(bot: &StarboardBot, url: &str, token: &str) -> reqwest::Result<serde_json::Value> {
    let ret = bot.reqwest.get(url).bearer_auth(token).send().await?;

    ret.json().await
}
