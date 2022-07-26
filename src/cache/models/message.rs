use twilight_model::{
    channel::{embed::Embed, Attachment, Message},
    id::{marker::UserMarker, Id},
};

pub struct CachedMessage {
    pub author_id: Id<UserMarker>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
}

impl From<Message> for CachedMessage {
    fn from(msg: Message) -> Self {
        Self {
            author_id: msg.author.id,
            attachments: msg.attachments,
            embeds: msg.embeds,
        }
    }
}
