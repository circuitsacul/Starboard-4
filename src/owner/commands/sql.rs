use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{client::bot::StarboardBot, owner::code_block::parse_code_blocks};

pub async fn run_sql(bot: &StarboardBot, event: &MessageCreate) -> anyhow::Result<()> {
    let blocks = parse_code_blocks(&event.content.strip_prefix("star sql").unwrap());
    println!("{:#?}", blocks);
    Ok(())
}
