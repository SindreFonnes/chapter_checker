use super::CheckError;
use crate::common_fn::get_site_as_string;
use crate::data::Entry;
use regex::Regex;

pub async fn check(entry: &Entry) -> Result<(String, f32), CheckError> {
    let text = get_site_as_string(&entry.url)
        .await
        .map_err(|err| CheckError::Request(err))?;

    let text: Vec<&str> = text.split("chapterlist").collect();

    let text: Vec<_> = text[1].split("</li>").collect();

    let re = Regex::new(r"Chapter ([0-9,.]*)").unwrap();

    let text = re
        .captures(text[0])
        .and_then(|captures| captures.get(1))
        .map(|m| m.as_str())
        .ok_or(CheckError::Parse(format!(
            "Couldn't find the chapter string asura {}",
            entry.name
        )))?;

    let chapter = text
        .parse::<f32>()
        .unwrap();

    Ok((entry.url.clone(), chapter))
}
