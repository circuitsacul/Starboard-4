use twilight_model::{
    channel::{embed::Embed, Attachment as ReceivedAttachment},
    http::attachment::Attachment,
};
use twilight_util::builder::embed::{EmbedBuilder, ImageSource};

pub struct AttachmentHandle {
    pub filename: String,
    pub content_type: Option<String>,
    pub url: String,
    pub proxy_url: String,
}

impl AttachmentHandle {
    pub fn into_attachment(self, id: u64) -> Attachment {
        Attachment::from_bytes(self.filename, b"hello".to_vec(), id)
    }

    pub fn from_attachment(attachment: &ReceivedAttachment) -> Self {
        Self {
            filename: attachment.filename.clone(),
            content_type: attachment.content_type.clone(),
            url: attachment.url.clone(),
            proxy_url: attachment.proxy_url.clone(),
        }
    }

    pub fn as_embed(&self) -> Option<Embed> {
        if let Some(image) = self.embedable_image() {
            Some(EmbedBuilder::new().image(image).validate().unwrap().build())
        } else {
            None
        }
    }

    pub fn url_list_item(&self) -> String {
        self.proxy_url.clone()
    }

    pub fn embedable_image(&self) -> Option<ImageSource> {
        Some(ImageSource::url(&self.url).unwrap())
    }
}

pub trait VecAttachments {
    fn into_attachments(self) -> Vec<Attachment>;
}

impl VecAttachments for Vec<AttachmentHandle> {
    fn into_attachments(self) -> Vec<Attachment> {
        let mut attachments = Vec::new();
        let mut current_id = 0;
        for attachment in self {
            attachments.push(attachment.into_attachment(current_id));
            current_id += 1;
        }
        attachments
    }
}
