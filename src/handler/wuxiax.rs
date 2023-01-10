use super::CheckError;
use regex::Regex;

pub fn check(text: String, url: String) -> Result<f32, CheckError> {
    let text: Vec<&str> = text.split("<strong>Latest").collect();

    let text: Vec<_> = text[1].split("</a>").collect();

    let text = text[0].to_lowercase();

    let text: Vec<_> = text.split("\">chapter").collect();

    let text = text[1];

    let re = Regex::new(r" ([0-9]*)").unwrap();

    let text = re
        .captures(text)
        .and_then(|captures| captures.get(1))
        .map(|m| m.as_str())
        .ok_or(CheckError::Parse(format!(
            "Couldn't find that chapter string wuxiax {}",
            url
        )))?;

    let chapter = text.parse::<f32>().unwrap();

    Ok(chapter)
}
