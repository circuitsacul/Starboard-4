use twilight_model::{
    application::component::{button::ButtonStyle, ActionRow, Button, Component},
    id::{marker::UserMarker, Id},
};

use crate::client::bot::StarboardBot;

use super::dm;

pub async fn notify(bot: &StarboardBot, user_id: Id<UserMarker>, message: &str) {
    if bot.config.development {
        println!("Development, skipping notification:");
        println!("{}", message);
        return;
    }

    let create = dm::dm(bot, user_id).await;
    let create = match create {
        Err(_) => return,
        Ok(create) => create,
    };

    let comp = Component::ActionRow(ActionRow {
        components: vec![Component::Button(Button {
            label: Some("Dismiss".to_string()),
            url: None,
            style: ButtonStyle::Secondary,
            custom_id: Some("stateless::dismiss_notification".to_string()),
            disabled: false,
            emoji: None,
        })],
    });

    let _ = create
        .content(message)
        .unwrap()
        .components(&[comp])
        .unwrap()
        .exec()
        .await;
}
