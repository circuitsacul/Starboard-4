use std::sync::Arc;

use twilight_model::{
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component,
    },
    id::{
        marker::{MessageMarker, UserMarker},
        Id,
    },
};

use crate::{
    client::bot::StarboardBot,
    errors::StarboardResult,
    interactions::context::{CommandCtx, ComponentCtx},
};

use super::wait_for::wait_for_button;

pub fn components(danger: bool) -> Vec<Component> {
    vec![Component::ActionRow(ActionRow {
        components: vec![
            Component::Button(Button {
                custom_id: Some("stateless::confirm_no".to_string()),
                disabled: false,
                emoji: None,
                label: Some("Cancel".to_string()),
                style: ButtonStyle::Secondary,
                url: None,
            }),
            Component::Button(Button {
                custom_id: Some("stateless::confirm_yes".to_string()),
                disabled: false,
                emoji: None,
                label: Some("Confirm".to_string()),
                style: if danger {
                    ButtonStyle::Danger
                } else {
                    ButtonStyle::Primary
                },
                url: None,
            }),
        ],
    })]
}

pub async fn wait_for_result(
    bot: Arc<StarboardBot>,
    message_id: Id<MessageMarker>,
    user_id: Id<UserMarker>,
) -> Option<(ComponentCtx, bool)> {
    let int_ctx = wait_for_button(
        bot,
        &["stateless::confirm_yes", "stateless::confirm_no"],
        message_id,
        user_id,
    )
    .await?;

    let conf = match &*int_ctx.data.custom_id {
        "stateless::confirm_yes" => true,
        "stateless::confirm_no" => false,
        _ => unreachable!(),
    };

    Some((int_ctx, conf))
}

pub async fn simple(
    ctx: &mut CommandCtx,
    prompt: &str,
    danger: bool,
) -> StarboardResult<Option<ComponentCtx>> {
    let cmd = ctx
        .build_resp()
        .content(prompt)
        .components(components(danger))
        .build();
    let msg = ctx.respond(cmd).await?.model().await.unwrap();
    let ret = wait_for_result(
        ctx.bot.clone(),
        msg.id,
        ctx.interaction.author_id().unwrap(),
    )
    .await;

    if let Some((mut int_ctx, conf)) = ret {
        if conf {
            return Ok(Some(int_ctx));
        } else {
            int_ctx.edit_str("Canceled.", true).await?;
        }
    } else {
        ctx.bot
            .http
            .update_message(msg.channel_id, msg.id)
            .content(Some("Canceled."))
            .unwrap()
            .components(Some(&[]))
            .unwrap()
            .await?;
    }

    Ok(None)
}
