use twilight_model::{
    channel::{message::embed::Embed, Attachment, Message},
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
            referenced_message: msg.reference.as_ref().and_then(|r| r.message_id),
        }
    }
}
