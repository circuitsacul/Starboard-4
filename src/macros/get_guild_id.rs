/// Get the guild_id from a CmdContext, sending an error to the user
/// if it is nonexistent (meaning the command was run inside DMs).
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
