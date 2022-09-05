use crate::interactions::context::ComponentCtx;

use super::dismiss::handle_dismiss;

pub async fn handle_component(ctx: ComponentCtx) -> anyhow::Result<()> {
    match ctx.data.custom_id.as_str() {
        "stateless::dismiss_notification" => handle_dismiss(&ctx).await?,
        _ => {}
    }

    Ok(())
}
