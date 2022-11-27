use regex::Regex;
use crate::data::{Site};
use crate::common_fn::{get_site_as_string};

pub async fn check_manga_site(site: &Site) -> Result<f32, reqwest::Error> {
    let text = get_site_as_string(&site.url).await?;

    let text: Vec<&str> = text
        .lines()
        .filter(|segment| segment.contains("chapter"))
        .filter(|segment| segment.contains("</a>") && segment.len() < 2000)
        .collect();

    let text = text[0].split(">").skip(1).next().unwrap().to_lowercase();

    let text: Vec<_> = text.split("chapter").collect();

    let re = Regex::new(r"[^\d.]").unwrap();

    let final_text = re
        .replace_all(text[1], "")
        .to_string()
        .parse::<f32>()
        .unwrap();

    println!("{:#?}", final_text);
	Ok(final_text)
}
