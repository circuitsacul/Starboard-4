use twilight_model::channel::message::{
    embed::{Embed, EmbedImage},
    sticker::StickerFormatType,
};
use twilight_util::builder::embed::ImageSource;

use crate::cache::models::message::CachedMessage;

use super::{
    image_only_embed::maybe_get_attachment_handle,
    imgur::{modify_imgur_embed, ImgurResult},
    youtube::modify_yt_embed,
    AttachmentHandle,
};

pub type StickerNames = String;
pub type PrimaryImage = ImageSource;
pub type Embeds = Vec<Embed>;
pub type UploadAttachments = Vec<AttachmentHandle>;

#[derive(Default)]
pub struct AttachmentUrls {
    pub embedded: Vec<String>,
    pub uploaded: Vec<String>,
}

pub struct ParsedMessage {
    pub sticker_names_str: Option<String>,
    // attachments
    pub urls: AttachmentUrls,
    pub primary_image: Option<ImageSource>,
    pub embeds: Vec<Embed>,
    pub upload_attachments: Vec<AttachmentHandle>,
}

impl ParsedMessage {
    pub fn parse(orig: &CachedMessage) -> Self {
        let (sticker_names_str, primary_image, url_list, embeds, upload_attachments) =
            Self::parse_attachments(orig);

        Self {
            sticker_names_str,
            primary_image,
            urls: url_list,
            embeds,
            upload_attachments,
        }
    }

    pub fn parse_attachments(
        orig: &CachedMessage,
    ) -> (
        Option<StickerNames>,
        Option<PrimaryImage>,
        AttachmentUrls,
        Embeds,
        UploadAttachments,
    ) {
        let mut primary_image = None;
        let mut embeds = Vec::new();
        let mut upload_attachments = Vec::new();
        let mut urls = AttachmentUrls::default();

        for attachment in &orig.attachments {
            let handle = AttachmentHandle::from_attachment(attachment);

            if primary_image.is_none() {
                if let Some(image) = handle.embedable_image() {
                    urls.embedded.push(handle.url_list_item());
                    primary_image.replace(image);
                    continue;
                }
            } else if let Some(embed) = handle.as_embed() {
                urls.embedded.push(handle.url_list_item());
                embeds.push(embed);
                continue;
            }

            urls.uploaded.push(handle.url_list_item());
            upload_attachments.push(handle);
        }

        for embed in &orig.embeds {
            if let Some(provider) = &embed.provider {
                if matches!(provider.name.as_deref(), Some("Imgur")) {
                    let ret = modify_imgur_embed(embed.clone());

                    match ret {
                        ImgurResult::Video(attachment) => upload_attachments.push(attachment),
                        ImgurResult::Image(embed) => embeds.push(*embed),
                    }

                    continue;
                }
            }

            if let Some(attachment) = maybe_get_attachment_handle(embed) {
                if let Some(image) = attachment.embedable_image() {
                    if primary_image.is_none() && embeds.is_empty() {
                        primary_image.replace(image);
                    } else {
                        embeds.push(attachment.as_embed().unwrap());
                    }
                } else {
                    upload_attachments.push(attachment);
                }
            } else {
                let mut embed = embed.to_owned();

                if &*embed.kind == "article" && embed.image.is_none() {
                    let thumb = std::mem::take(&mut embed.thumbnail);
                    if let Some(thumb) = thumb {
                        embed.image = Some(EmbedImage {
                            height: None,
                            width: None,
                            proxy_url: None,
                            url: thumb.url,
                        });
                    }
                }

                if let Some(provider) = &embed.provider {
                    if let Some(name) = &provider.name {
                        match name.as_str() {
                            "YouTube" => modify_yt_embed(&mut embed),
                            "FixTweet" => {
                                embed.description = None;
                            }
                            _ => (),
                        }
                    }
                }

                embeds.push(embed);
            }
        }

        let sticker_names_str: Option<String>;
        if !orig.stickers.is_empty() {
            let mut sticker_names = Vec::new();

            for sticker in &orig.stickers {
                match sticker.format_type {
                    StickerFormatType::Lottie => {
                        sticker_names.push(format!("Sticker: **{}**", sticker.name));
                    }
                    StickerFormatType::Apng | StickerFormatType::Png => {
                        let handle = AttachmentHandle {
                            filename: format!("{}.png", sticker.name),
                            content_type: Some("image/png".to_string()),
                            url: format!("https://cdn.discordapp.com/stickers/{}.png", sticker.id),
                        };

                        if primary_image.is_none() {
                            if let Some(image) = handle.embedable_image() {
                                primary_image.replace(image);
                                continue;
                            }
                        }

                        if let Some(embed) = handle.as_embed() {
                            embeds.push(embed);
                            continue;
                        }

                        upload_attachments.push(handle);
                    }
                    StickerFormatType::Unknown(format) => {
                        eprintln!("Unkown sticker format type {format}.")
                    }
                    unhandled => {
                        eprintln!("Twilight added sticker format type {unhandled:?}.");
                    }
                }
            }

            if sticker_names.is_empty() {
                sticker_names_str = None;
            } else {
                sticker_names_str = Some(sticker_names.join("\n"))
            }
        } else {
            sticker_names_str = None;
        }

        (
            sticker_names_str,
            primary_image,
            urls,
            embeds,
            upload_attachments,
        )
    }
}
