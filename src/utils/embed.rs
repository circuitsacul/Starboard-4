use twilight_util::builder::embed as tw_embed;

use crate::constants;

pub fn build() -> tw_embed::EmbedBuilder {
    tw_embed::EmbedBuilder::new().color(constants::BOT_COLOR)
}
