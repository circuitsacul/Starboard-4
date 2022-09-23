use twilight_model::channel::embed::Embed;

use crate::utils::gifv::get_gif_url;

use super::AttachmentHandle;

pub fn maybe_get_attachment_handle(embed: &Embed) -> Option<AttachmentHandle> {
    // gifs
    if &embed.kind == "gifv"
        && matches!(
            embed.provider.as_ref().and_then(|p| p.name.as_deref()),
            Some("Tenor") | Some("Giphy")
        )
    {
        let url = get_gif_url(
            &embed.thumbnail.as_ref().unwrap().url,
            embed.provider.as_ref().unwrap().name.as_ref().unwrap(),
        );

        if let Some(url) = url {
            let attachment = AttachmentHandle {
                filename: "GIF".to_string(),
                content_type: Some("image/gif".to_string()),
                url,
            };

            return Some(attachment);
        }
    }

    // catch-all logic
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
