use twilight_model::{channel::embed::Embed, http::attachment::Attachment};
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::{cache::models::message::CachedMessage, constants};

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
        let parsed = ParsedMessage::parse(handle, orig);

        Self::Full(FullBuiltStarboardEmbed {
            top_content: Self::build_top_content(handle),
            embeds: Self::build_embeds(handle, orig, &parsed),
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

    pub fn build_embeds(
        handle: &Embedder,
        orig: &CachedMessage,
        parsed: &ParsedMessage,
    ) -> Vec<Embed> {
        let primary_embed = Self::build_primary_embed(handle, orig, parsed);
        let mut embeds = match primary_embed {
            None => Vec::new(),
            Some(embed) => vec![embed],
        };
        if handle.config.resolved.extra_embeds {
            for e in &parsed.embedded_images {
                embeds.push(e.clone());
            }
            for e in &orig.embeds {
                embeds.push(e.clone());
            }
        }
        embeds
    }

    pub fn build_primary_embed(
        handle: &Embedder,
        orig: &CachedMessage,
        parsed: &ParsedMessage,
    ) -> Option<Embed> {
        let mut embed = EmbedBuilder::new().color(
            handle
                .config
                .resolved
                .color
                .map(|c| c as u32)
                .unwrap_or(constants::BOT_COLOR),
        );

        // main description
        {
            let content = if orig.raw_content.is_empty() {
                "*nothing to display*".to_string()
            } else {
                orig.raw_content.clone()
            };
            embed = embed.description(content);
        }

        // attachments list
        if handle.config.resolved.attachments_list && !parsed.url_list.is_empty() {
            embed = embed
                .field(EmbedFieldBuilder::new(constants::ZWS, parsed.url_list.join("\n")).build())
        }

        // primary image
        if let Some(image) = &parsed.primary_image {
            embed = embed.image(image.clone());
        }

        // build
        let embed = embed.build();

        let is_empty = {
            let is_desc_empty = match &embed.description {
                None => true,
                Some(desc) => desc.is_empty(),
            };

            is_desc_empty && embed.fields.is_empty()
        };

        if is_empty {
            None
        } else {
            Some(embed)
        }
    }
}
