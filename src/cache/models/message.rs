use twilight_model::{
    channel::{
        message::{embed::Embed, sticker::MessageSticker},
        Attachment, Message,
    },
    id::{
        marker::{MessageMarker, UserMarker},
        Id,
    },
};

use crate::utils::system_content::SystemContent;

pub struct CachedMessage {
    pub author_id: Id<UserMarker>,
    pub content: String,
    pub attachments: Vec<Attachment>,
    pub stickers: Vec<MessageSticker>,
    pub embeds: Vec<Embed>,
    pub referenced_message: Option<Id<MessageMarker>>,
}

impl From<Message> for CachedMessage {
    fn from(msg: Message) -> Self {
        let content = msg.system_content();
        Self {
            author_id: msg.author.id,
            attachments: msg.attachments,
            embeds: msg.embeds,
            content,
            stickers: msg.sticker_items,
            referenced_message: msg.reference.as_ref().and_then(|r| r.message_id),
        }
    }
}

impl From<&Message> for CachedMessage {
    fn from(msg: &Message) -> Self {
        Self {
            author_id: msg.author.id,
            attachments: msg.attachments.clone(),
            embeds: msg.embeds.clone(),
            content: msg.system_content(),
            stickers: msg.sticker_items.clone(),
            referenced_message: msg.reference.as_ref().and_then(|r| r.message_id),
        }
    }
}
