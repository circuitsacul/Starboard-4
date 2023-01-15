use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    database::Member, errors::StarboardResult, interactions::context::CommandCtx,
    utils::id_as_i64::GetI64,
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "disable", desc = "Disable autoredeem for a server.")]
pub struct Disable {
    /// The server to disable autoredeem for.
    #[command(autocomplete = true)]
    server: Option<String>,
}

impl Disable {
    pub async fn callback(self, mut ctx: CommandCtx) -> StarboardResult<()> {
        let user_id = ctx.interaction.author_id().unwrap().get_i64();

        let guild_id = 'out: {
            let Some(input_guild) = self.server else {
                let Some(guild_id) = ctx.interaction.guild_id else {
                    ctx.respond_str(
                        "Please specify a server, or run this command inside one.",
                        true
                    ).await?;
                    return Ok(());
                };

                break 'out guild_id.get_i64();
            };

            let Ok(guild_id) = input_guild.parse::<i64>() else {
                ctx.respond_str(
                    "Please entire a server ID, or select a server from the options.", 
                    true
                ).await?;
                return Ok(());
            };

            guild_id
        };

        // if self.enabled {
        //     let is_member = ctx.bot.cache.guilds.with(&guild_id.into_id(), |_, guild| {
        //         let Some(guild) = guild else { return false; };

        //         guild.members.contains_key(&user_id.into_id())
        //     });

        //     if !is_member {
        //         ctx.respond_str(
        //             "You cannot enable autoredeem for a server you are not in.",
        //             true,
        //         )
        //         .await?;
        //         return Ok(());
        //     }

        //     map_dup_none!(Member::create(&ctx.bot.pool, user_id, guild_id))?;
        // }

        Member::set_autoredeem_enabled(&ctx.bot.pool, user_id, guild_id, false).await?;

        ctx.respond_str("Autoredeem disabled.", true).await?;

        Ok(())
    }
}
