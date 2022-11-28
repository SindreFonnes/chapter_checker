use super::CheckError;
use crate::common_fn::get_site_as_string;
use crate::data::Entry;
use regex::Regex;

pub async fn check(entry: &Entry) -> Result<(String, f32), CheckError> {
    let text = get_site_as_string(&entry.url)
        .await
        .map_err(|err| CheckError::Request(err))?;

    let text: Vec<&str> = text
        .lines()
        .filter(|segment| segment.contains("chapter"))
        .filter(|segment| segment.contains("</a>") && segment.len() < 2000)
        .collect();

    let text = text[0].split(">").skip(1).next().unwrap().to_lowercase();

    let text: Vec<_> = text.split("chapter").collect();

    let re = Regex::new(r"[^\d.]").unwrap();

    let chapter = re
        .replace_all(text[1], "")
        .to_string()
        .parse::<f32>()
        .map_err(|err| CheckError::Parse(format!("Couldn't parse float manganato {err} {}", entry.name)))?;

    
    Ok((entry.url.clone(), chapter))
}
