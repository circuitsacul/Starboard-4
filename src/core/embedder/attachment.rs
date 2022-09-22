use async_trait::async_trait;
use twilight_model::{
    channel::{embed::Embed, Attachment as ReceivedAttachment},
    http::attachment::Attachment,
};
use twilight_util::builder::embed::{EmbedBuilder, ImageSource};

use crate::{
    constants,
    errors::{StarboardError, StarboardResult},
};

pub struct AttachmentHandle {
    pub filename: String,
    pub content_type: Option<String>,
    pub url: String,
}

impl AttachmentHandle {
    pub async fn as_attachment(&self, id: u64) -> StarboardResult<Attachment> {
        let file = reqwest::get(&self.url).await?.bytes().await?;

        Ok(Attachment::from_bytes(
            self.filename.clone(),
            file.to_vec(),
            id,
        ))
    }

    pub fn from_attachment(attachment: &ReceivedAttachment) -> Self {
        Self {
            filename: attachment.filename.clone(),
            content_type: attachment.content_type.clone(),
            url: attachment.url.clone(),
        }
    }

    pub fn as_embed(&self) -> Option<Embed> {
        self.embedable_image().map(|image| {
            EmbedBuilder::new()
                .image(image)
                .color(constants::EMBED_DARK_BG)
                .build()
        })
    }

    pub fn url_list_item(&self) -> String {
        format!("[{}]({})", self.filename, self.url)
    }

    pub fn embedable_image(&self) -> Option<ImageSource> {
        if let Some(ct) = &self.content_type {
            if ct.starts_with("image") {
                return Some(ImageSource::url(&self.url).unwrap());
            }
        }

        None
    }
}

#[async_trait]
pub trait VecAttachments {
    async fn as_attachments(&self) -> (Vec<Attachment>, Vec<StarboardError>);
}

#[async_trait]
impl VecAttachments for Vec<AttachmentHandle> {
    async fn as_attachments(&self) -> (Vec<Attachment>, Vec<StarboardError>) {
        let mut attachments = Vec::new();
        let mut errors = Vec::new();
        for (current_id, attachment) in self.iter().enumerate() {
            match attachment.as_attachment(current_id as u64).await {
                Err(why) => errors.push(why),
                Ok(file) => attachments.push(file),
            }
        }
        (attachments, errors)
    }
}
