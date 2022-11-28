use super::CheckError;
use crate::common_fn::get_site_as_string;
use crate::data::Entry;
use regex::Regex;

pub async fn check(entry: &Entry) -> Result<(String, f32), CheckError> {
    let text = get_site_as_string(&entry.url)
        .await
        .map_err(|err| CheckError::Request(err))?;

    let text: Vec<&str> = text.split("<strong>Latest").collect();

    let text: Vec<_> = text[1].split("</a>").collect();

    let text = text[0].to_lowercase();

    let text: Vec<_> = text.split("\">chapter").collect();

    let text = text[1];

    let re = Regex::new(r" ([0-9]*)").unwrap();

    let text = re
        .captures(text)
        .and_then(|captures| captures.get(1))
        .map(|m| {
            println!("{}", m.as_str());
            m.as_str()
        })
        .ok_or(CheckError::Parse(format!(
            "Couldn't find that chapter string professor {}",
            entry.name
        )))?;

    let chapter = text.parse::<f32>().unwrap();

    Ok((entry.url.clone(), chapter))
}
