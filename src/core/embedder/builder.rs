use std::fmt::Write;

use twilight_model::{
    channel::message::embed::Embed,
    id::{marker::MessageMarker, Id},
    util::Timestamp,
};
use twilight_util::{
    builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource},
    snowflake::Snowflake,
};

use crate::{cache::models::message::CachedMessage, constants, unwrap_id, utils::into_id::IntoId};

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
        let orig = match &handle.orig_message {
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
        let ref_msg = match &handle.referenced_message {
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
        let mut embed_is_empty = true;
        let mut zws_fields: Vec<String> = Vec::new();
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
            let avatar: Option<String>;
            let name: String;
            (name, avatar) = match maybe_user {
                None => ("Deleted User".to_string(), None),
                Some(user) => {
                    let member = handle.bot.cache.guilds.with(
                        &handle.config.starboard.guild_id.into_id(),
                        |_, g| {
                            if let Some(g) = g {
                                g.value().members.get(&orig.author_id).map(|m| (*m).clone())
                            } else {
                                None
                            }
                        },
                    );

                    let (name, avatar) = match &member {
                        Some(member) => (
                            member.nickname.as_ref().unwrap_or(&user.name).to_owned(),
                            member
                                .server_avatar_url
                                .as_ref()
                                .or(user.avatar_url.as_ref())
                                .cloned(),
                        ),
                        None => (user.name.clone(), user.avatar_url.as_ref().cloned()),
                    };
                    (name, avatar)
                }
            };
            let name = if is_reply {
                format!("Replying to {}", name)
            } else {
                name
            };

            let mut author = EmbedAuthorBuilder::new(name).url(&link);
            if let Some(avatar) = avatar {
                author = author.icon_url(ImageSource::url(avatar).unwrap());
            }

            embed = embed.author(author.build())
        }

        // main description
        let mut has_description;
        if !orig.content.is_empty() {
            embed_is_empty = false;
            embed = embed.description(&orig.content);
            has_description = true;
        } else {
            has_description = false;
        }

        // jump link
        if handle.config.resolved.jump_to_message && !is_reply {
            embed_is_empty = false;
            zws_fields.push(format!("[Go to Message]({})", link));
        }

        // attachments list
        if (handle.config.resolved.attachments_list || is_reply) && !parsed.url_list.is_empty() {
            embed_is_empty = false;
            zws_fields.push(parsed.url_list.join("\n"));
        }

        // primary image
        if let Some(image) = &parsed.primary_image {
            embed_is_empty = false;
            embed = embed.image(image.clone());
        }

        // timestamp
        {
            let id: Id<MessageMarker> = handle.orig_sql_message.message_id.into_id();
            embed = embed.timestamp(Timestamp::from_micros(id.timestamp() * 1000).unwrap());
        }

        // add the fields
        for field in zws_fields {
            if !has_description {
                has_description = true;
                embed = embed.description(field);
            } else {
                embed = embed.field(EmbedFieldBuilder::new(constants::ZWS, field).build());
            }
        }

        // placeholder content, if needed
        if embed_is_empty {
            embed = embed.description("*file only*");
        }

        // build
        Some(embed.build())
    }
}
