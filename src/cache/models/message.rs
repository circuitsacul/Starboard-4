use twilight_model::{
    channel::{embed::Embed, Attachment},
    id::{marker::MessageMarker, Id},
};

pub struct CachedMessage {
    pub id: Id<MessageMarker>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
}
