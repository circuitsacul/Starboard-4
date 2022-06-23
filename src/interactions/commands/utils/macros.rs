#[macro_export]
macro_rules! get_guild_id {
    ($ctx: expr) => {
        match $ctx.command.guild_id {
            None => {
                $ctx.respond_str("This command must be used in a server.", true)
                    .await?;
                return Ok(());
            }
            Some(value) => value.get().try_into().unwrap(),
        }
    };
}

#[macro_export]
macro_rules! assert_channel_is_textable {
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
