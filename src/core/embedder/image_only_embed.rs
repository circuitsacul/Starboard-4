use twilight_model::channel::embed::Embed;

use super::AttachmentHandle;

pub fn maybe_get_attachment_handle(embed: &Embed) -> Option<AttachmentHandle> {
    if !(embed.author.is_none()
        && embed.title.is_none()
        && embed.description.is_none()
        && embed.fields.is_empty()
        && (embed.image.is_some() ^ embed.thumbnail.is_some()))
    {
        return None;
    }

    let (url, ct) = if let Some(image) = &embed.image {
        (image.url.clone(), embed.kind.clone())
    } else if let Some(image) = &embed.thumbnail {
        (image.url.clone(), embed.kind.clone())
    } else {
        unreachable!()
    };

    let attachment = AttachmentHandle {
        filename: "Embed Image".to_string(),
        content_type: Some(format!("image/{}", ct)),
        url,
    };

    Some(attachment)
}
