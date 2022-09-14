use crate::interactions::context::ComponentCtx;

use super::dismiss::handle_dismiss;

pub async fn handle_component(ctx: ComponentCtx) -> anyhow::Result<()> {
    if let "stateless::dismiss_notification" = ctx.data.custom_id.as_str() {
        handle_dismiss(&ctx).await?
    }

    Ok(())
}
