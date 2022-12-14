use crate::{errors::StarboardResult, interactions::context::ComponentCtx};

use super::dismiss::handle_dismiss;

pub async fn handle_component(ctx: ComponentCtx) -> StarboardResult<()> {
    #[allow(clippy::single_match)]
    match ctx.data.custom_id.as_str() {
        "stateless::dismiss_notification" => handle_dismiss(&ctx).await?,
        _ => {}
    }

    Ok(())
}
