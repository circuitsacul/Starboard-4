use std::fmt::Write;

use twilight_model::channel::embed::Embed;
use twilight_util::builder::embed::{
    EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource,
};

use crate::{cache::models::message::CachedMessage, constants};

use super::{parser::ParsedMessage, AttachmentHandle, Embedder};

pub struct FullBuiltStarboardEmbed {
    pub top_content: String,
    pub embeds: Vec<Embed>,
    pub embedded_images: Vec<Embed>,
    pub upload_attachments: Vec<AttachmentHandle>,
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
            upload_attachments: parsed.upload_attachments,
        })
    }

    pub fn build_top_content(handle: &Embedder) -> String {
        let mut top_content = String::new();

        if let Some(ref emoji) = handle.config.resolved.display_emoji {
            top_content.push_str(emoji);
        }
        write!(
            top_content,
            " **{} |** <#{}>",
            handle.points, handle.orig_sql_message.channel_id,
        )
        .unwrap();
        if handle.config.resolved.ping_author {
            write!(
                top_content,
                " **(**<@{}>**)**",
                handle.orig_sql_message.author_id
            )
            .unwrap();
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

        // author
        {
            let avatar: Option<&str>;
            let name;
            (name, avatar) = match &handle.orig_message_author {
                None => ("Deleted User", None),
                Some(user) => (user.name.as_str(), user.avatar_url.as_deref()),
            };

            let mut author = EmbedAuthorBuilder::new(name);
            if let Some(avatar) = avatar {
                author = author.icon_url(ImageSource::url(avatar).unwrap());
            }

            embed = embed.author(author.build())
        }

        // main description
        {
            let content = if orig.content.is_empty() {
                "*file only*"
            } else {
                &orig.content
            };
            embed = embed.description(content);
        }

        // jump link
        if handle.config.resolved.jump_to_message {
            let link = format!(
                "[Go to Message](https://discord.com/channels/{}/{}/{})",
                handle.config.starboard.guild_id,
                handle.orig_sql_message.channel_id,
                handle.orig_sql_message.message_id
            );
            embed = embed.field(EmbedFieldBuilder::new(constants::ZWS, link));
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
