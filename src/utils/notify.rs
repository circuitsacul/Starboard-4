use twilight_model::id::{marker::UserMarker, Id};

use crate::client::bot::StarboardBot;

use super::dm;

pub async fn notify(bot: &StarboardBot, user_id: Id<UserMarker>, message: &str) -> () {
    let create = dm::dm(bot, user_id).await;
    let create = match create {
        Err(_) => return,
        Ok(create) => create,
    };

    let _ = create.content(message).unwrap().exec().await;
}
