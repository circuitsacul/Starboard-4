use twilight_model::channel::{embed::Embed, Attachment};

pub struct CachedMessage {
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
}
