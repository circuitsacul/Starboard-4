use twilight_model::channel::embed::Embed;
use twilight_util::builder::embed::ImageSource;

use crate::cache::models::message::CachedMessage;

use super::{image_only_embed::maybe_get_attachment_handle, AttachmentHandle, Embedder};

pub struct ParsedMessage {
    // attachments
    pub url_list: Vec<String>,
    pub primary_image: Option<ImageSource>,
    pub embeds: Vec<Embed>,
    pub upload_attachments: Vec<AttachmentHandle>,
}

impl ParsedMessage {
    pub fn parse(_handle: &Embedder, orig: &CachedMessage) -> Self {
        let (primary_image, url_list, embeds, upload_attachments) = Self::parse_attachments(orig);

        Self {
            primary_image,
            url_list,
            embeds,
            upload_attachments,
        }
    }

    pub fn parse_attachments(
        orig: &CachedMessage,
    ) -> (
        Option<ImageSource>,
        Vec<String>,
        Vec<Embed>,
        Vec<AttachmentHandle>,
    ) {
        let mut primary_image = None;
        let mut embeds = Vec::new();
        let mut upload_attachments = Vec::new();
        let mut url_list = Vec::new();

        for attachment in &orig.attachments {
            let handle = AttachmentHandle::from_attachment(attachment);
            url_list.push(handle.url_list_item());

            if primary_image.is_none() {
                if let Some(image) = handle.embedable_image() {
                    primary_image.replace(image);
                    continue;
                }
            } else if let Some(embed) = handle.as_embed() {
                embeds.push(embed);
                continue;
            }

            upload_attachments.push(handle);
        }

        for embed in &orig.embeds {
            if let Some(attachment) = maybe_get_attachment_handle(embed) {
                if primary_image.is_none() {
                    primary_image.replace(attachment.embedable_image().unwrap());
                } else {
                    embeds.push(attachment.as_embed().unwrap());
                }
            } else {
                embeds.push(embed.clone());
            }
        }

        (primary_image, url_list, embeds, upload_attachments)
    }
}
