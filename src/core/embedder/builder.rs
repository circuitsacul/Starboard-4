use twilight_model::{channel::embed::Embed, http::attachment::Attachment};
use twilight_util::builder::embed::EmbedBuilder;

use crate::cache::models::message::CachedMessage;

use super::{attachment::VecAttachments, parser::ParsedMessage, Embedder};

pub struct FullBuiltStarboardEmbed {
    pub top_content: String,
    pub embeds: Vec<Embed>,
    pub embedded_images: Vec<Embed>,
    pub upload_attachments: Vec<Attachment>,
}

pub struct PartialBuiltStarboardEmbed {
    pub top_content: String,
}

pub enum BuiltStarboardEmbed {
    Full(FullBuiltStarboardEmbed),
    Partial(PartialBuiltStarboardEmbed),
}

impl BuiltStarboardEmbed {
    pub fn build(handle: &Embedder) -> Self {
        let orig = match &*handle.orig_message {
            None => {
                return Self::Partial(PartialBuiltStarboardEmbed {
                    top_content: Self::build_top_content(handle),
                })
            }
            Some(orig) => orig,
        };
        let parsed = ParsedMessage::parse(handle, &orig);

        let embeds = vec![Self::build_primary_embed(handle, orig, &parsed)];
        Self::Full(FullBuiltStarboardEmbed {
            top_content: Self::build_top_content(handle),
            embeds,
            embedded_images: parsed.embedded_images,
            upload_attachments: parsed.upload_attachments.into_attachments(),
        })
    }

    pub fn build_top_content(handle: &Embedder) -> String {
        let mut top_content = String::new();

        if let Some(ref emoji) = handle.config.resolved.display_emoji {
            top_content.push_str(emoji);
        }
        top_content.push_str(&format!(
            " **{} |** <#{}>",
            handle.points, handle.orig_sql_message.channel_id,
        ));
        if handle.config.resolved.ping_author {
            top_content.push_str(&format!(
                " **(**<@{}>**)**",
                handle.orig_sql_message.author_id
            ));
        }

        top_content
    }

    pub fn build_primary_embed(
        handle: &Embedder,
        orig: &CachedMessage,
        parsed: &ParsedMessage,
    ) -> Embed {
        EmbedBuilder::new()
            .color(
                handle
                    .config
                    .resolved
                    .color
                    .map(|c| c.try_into().unwrap())
                    .unwrap_or(0),
            )
            .description(orig.raw_content.clone())
            .validate()
            .unwrap()
            .build()
    }
}
