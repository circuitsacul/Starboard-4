use twilight_model::channel::embed::Embed;
use twilight_util::builder::embed::ImageSource;

use crate::cache::models::message::CachedMessage;

use super::{AttachmentHandle, Embedder};

pub struct ParsedMessage {
    // attachments
    pub url_list: Vec<String>,
    pub primary_image: Option<ImageSource>,
    pub embedded_images: Vec<Embed>,
    pub upload_attachments: Vec<AttachmentHandle>,
}

impl ParsedMessage {
    pub fn parse(_handle: &Embedder, orig: &CachedMessage) -> Self {
        let (primary_image, url_list, embedded_images, upload_attachments) =
            Self::parse_attachments(orig);

        Self {
            primary_image,
            url_list,
            embedded_images,
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
        let mut embedded_images = Vec::new();
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
                embedded_images.push(embed);
                continue;
            }

            upload_attachments.push(handle);
        }

        (primary_image, url_list, embedded_images, upload_attachments)
    }
}
