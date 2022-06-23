/// "assert" that the channel is a "textable" guild channel,
/// sending an error to the user otherwise.
#[macro_export]
macro_rules! channel_is_textable {
    ($ctx: expr, $channel: expr) => {{
        use twilight_model::channel::ChannelType;
        let ret = match $channel.kind {
            ChannelType::GuildText => true,
            ChannelType::GuildNews => true,
            _ => false,
        };
        if !ret {
            $ctx.respond_str("Only textable channels are allowed.", true)
                .await?;
        }
    }};
}
