use lazy_static::lazy_static;

pub fn get_gif_url(url: &str, provider: &str) -> Option<String> {
    lazy_static! {
        static ref TENOR: regex::Regex =
            regex::Regex::new(r#"^https://media\.tenor\.com?/(\w+)/(.*)\.\w+$"#).unwrap();
    }
    lazy_static! {
        static ref GIPHY: regex::Regex =
            regex::Regex::new(r#"^https://media\d*\.giphy\.com/media/([\w-]+)/(.*)\.gif$"#)
                .unwrap();
    }

    match provider {
        "Tenor" => {
            let caps: Vec<_> = TENOR.captures_iter(url).collect();
            let groups = caps.get(0)?;

            let gif_id = &groups[1];
            let gif_id = gif_id[0..gif_id.len() - 1].to_string() + "C";
            let name = &groups[2];
            Some(format!("https://c.tenor.com/{}/{}.gif", gif_id, name))
        }
        "Giphy" => {
            let url = url.split('?').next().unwrap();
            let caps: Vec<_> = GIPHY.captures_iter(url).collect();
            let groups = caps.get(0)?;

            let gif_id = &groups[1];
            let name = &groups[2];
            let name = if name.ends_with("_s") {
                &name[0..name.len() - 2]
            } else {
                name
            };

            Some(format!("https://i.giphy.com/media/{}/{}.gif", gif_id, name))
        }
        other => {
            eprintln!("Unkown GIFV provider: {}\n{}", other, url);
            None
        }
    }
}
