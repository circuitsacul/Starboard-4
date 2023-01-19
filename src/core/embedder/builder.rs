use std::fmt::Write;

use twilight_model::{
    channel::message::embed::Embed,
    id::{marker::MessageMarker, Id},
    util::Timestamp,
};
use twilight_util::{
    builder::embed::{
        EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder, ImageSource,
    },
    snowflake::Snowflake,
};

use crate::{
    cache::models::message::CachedMessage,
    constants,
    core::emoji::{EmojiCommon, SimpleEmoji},
    utils::{id_as_i64::GetI64, into_id::IntoId, message_link::fmt_message_link},
};

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
    pub fn build(handle: &Embedder, force_partial: bool, watermark: bool) -> Self {
        if let Some(orig) = &handle.orig_message {
            if !force_partial {
                let parsed = ParsedMessage::parse(orig);

                return Self::Full(FullBuiltStarboardEmbed {
                    top_content: Self::build_top_content(handle),
                    embeds: Self::build_embeds(handle, orig, &parsed, watermark),
                    upload_attachments: parsed.upload_attachments,
                });
            }
        }

        Self::Partial(PartialBuiltStarboardEmbed {
            top_content: Self::build_top_content(handle),
        })
    }

    pub fn build_top_content(handle: &Embedder) -> String {
        let mut top_content = String::new();

        if let Some(emoji) = handle.config.resolved.display_emoji.clone() {
            let emoji = SimpleEmoji::from_stored(emoji);
            top_content.push_str(
                &emoji.into_readable(handle.bot, handle.config.starboard.guild_id.into_id()),
            );
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

        if handle.orig_sql_message.frozen {
            write!(top_content, " ❄️").unwrap();
        }
        if handle
            .orig_sql_message
            .forced_to
            .contains(&handle.config.starboard.id)
        {
            write!(top_content, " 🔒").unwrap();
        }

        top_content
    }

    pub fn build_embeds(
        handle: &Embedder,
        orig: &CachedMessage,
        parsed: &ParsedMessage,
        watermark: bool,
    ) -> Vec<Embed> {
        let mut embeds = Vec::new();

        if let Some(e) = Self::build_replied_embed(handle) {
            embeds.push(e);
        }
        if let Some(e) = Self::build_primary_embed(handle, orig, parsed, watermark, false) {
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
        let reply_parsed = ParsedMessage::parse(ref_msg);
        Self::build_primary_embed(handle, ref_msg, &reply_parsed, false, true)
    }

    pub fn build_primary_embed(
        handle: &Embedder,
        orig: &CachedMessage,
        parsed: &ParsedMessage,
        watermark: bool,
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

        let (link, mid_i64) = {
            let mid = match is_reply {
                true => handle
                    .orig_message
                    .as_ref()
                    .unwrap()
                    .referenced_message
                    .unwrap()
                    .get_i64(),
                false => handle.orig_sql_message.message_id,
            };

            (
                fmt_message_link(
                    handle.config.starboard.guild_id,
                    handle.orig_sql_message.channel_id,
                    mid,
                ),
                mid,
            )
        };
        let mid: Id<MessageMarker> = mid_i64.into_id();

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
                    let member =
                        handle.bot.cache.guilds.with(
                            &handle.config.starboard.guild_id.into_id(),
                            |_, guild| {
                                let guild = match guild {
                                    None => return None,
                                    Some(guild) => guild,
                                };

                                guild.value().members.get(&orig.author_id).map(|m| {
                                    (m.server_avatar_url.to_owned(), m.nickname.to_owned())
                                })
                            },
                        );

                    let (name, avatar) = match member {
                        Some((avatar_url, nickname)) => (
                            nickname.unwrap_or_else(|| user.name.to_owned()),
                            avatar_url.or_else(|| user.avatar_url.to_owned()),
                        ),
                        None => (user.name.to_owned(), user.avatar_url.to_owned()),
                    };
                    (name, avatar)
                }
            };
            let name = if is_reply {
                format!("Replying to {name}")
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
        let mut description = String::new();

        if let Some(name_str) = &parsed.sticker_names_str {
            description.push_str(name_str);
            description.push('\n');
        }

        if !orig.content.is_empty() {
            description.push_str(&orig.content);
        }

        let mut has_description;
        if !description.is_empty() {
            embed = embed.description(description);
            embed_is_empty = false;
            has_description = true;
        } else {
            has_description = false;
        }

        // jump link
        if handle.config.resolved.jump_to_message && !is_reply {
            embed_is_empty = false;
            zws_fields.push(format!("[Go to Message]({link})"));
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
        embed = embed.timestamp(Timestamp::from_micros(mid.timestamp() * 1000).unwrap());

        // add the fields
        for field in zws_fields {
            if !has_description {
                has_description = true;
                embed = embed.description(field);
            } else {
                embed = embed.field(EmbedFieldBuilder::new(constants::ZWS, field).build());
            }
        }

        // watermark footer
        if watermark {
            embed = embed.footer(EmbedFooterBuilder::new("Powered by https://starboard.best"));
        }

        // placeholder content, if needed
        if embed_is_empty {
            embed = embed.description("*file only*");
        }

        // build
        Some(embed.build())
    }
}
