use std::fmt::Write;

use twilight_model::channel::embed::Embed;
use twilight_util::builder::embed::{
    EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource,
};

use crate::{cache::models::message::CachedMessage, constants, unwrap_id};

use super::{parser::ParsedMessage, AttachmentHandle, Embedder};

pub struct FullBuiltStarboardEmbed {
    pub top_content: String,
    pub embeds: Vec<Embed>,
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
        let mut embeds = Vec::new();

        if let Some(e) = Self::build_replied_embed(handle) {
            embeds.push(e);
        }
        if let Some(e) = Self::build_primary_embed(handle, orig, parsed, false) {
            embeds.push(e);
        }

        if handle.config.resolved.extra_embeds {
            for e in &parsed.embeds {
                embeds.push(e.clone());
            }
        }

        embeds
    }

    pub fn build_replied_embed(handle: &Embedder) -> Option<Embed> {
        let ref_msg = match &*handle.referenced_message {
            None => return None,
            Some(msg) => msg,
        };
        let reply_parsed = ParsedMessage::parse(handle, ref_msg);
        Self::build_primary_embed(handle, ref_msg, &reply_parsed, true)
    }

    pub fn build_primary_embed(
        handle: &Embedder,
        orig: &CachedMessage,
        parsed: &ParsedMessage,
        is_reply: bool,
    ) -> Option<Embed> {
        let mut embed =
            EmbedBuilder::new().color(handle.config.resolved.color.map(|c| c as u32).unwrap_or(
                if is_reply {
                    constants::EMBED_DARK_BG
                } else {
                    constants::BOT_COLOR
                },
            ));

        let link = {
            let mid = match is_reply {
                true => unwrap_id!(handle
                    .orig_message
                    .as_ref()
                    .as_ref()
                    .unwrap()
                    .referenced_message
                    .unwrap()),
                false => handle.orig_sql_message.message_id,
            };

            format!(
                "https://discord.com/channels/{}/{}/{}",
                handle.config.starboard.guild_id, handle.orig_sql_message.channel_id, mid,
            )
        };

        // author
        {
            let maybe_user = match is_reply {
                true => &handle.referenced_message_author,
                false => &handle.orig_message_author,
            };
            let avatar: Option<&str>;
            let name;
            (name, avatar) = match maybe_user {
                None => ("Deleted User", None),
                Some(user) => (user.name.as_str(), user.avatar_url.as_deref()),
            };
            let name = if is_reply {
                format!("Replying to {}", name)
            } else {
                name.to_string()
            };

            let mut author = EmbedAuthorBuilder::new(name).url(&link);
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
        if handle.config.resolved.jump_to_message && !is_reply {
            embed = embed.field(EmbedFieldBuilder::new(
                constants::ZWS,
                format!("[Go to Message]({})", link),
            ));
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
        Some(embed.build())
    }
}
