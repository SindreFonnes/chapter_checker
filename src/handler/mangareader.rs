use crate::common_fn::parse_site_len_wrong;

use super::CheckError;
use regex::Regex;

pub fn check(text: String, url: String) -> Result<f32, CheckError> {
    let parse_error_message = format!("Error parsing mangareader: {}", url);

    let text = text
        .split("<div class=\"chapters-list-ul\">")
        .collect::<Vec<&str>>();

    parse_site_len_wrong(&text, parse_error_message.clone())?;

    let text = text[1].split("<a href=\"").collect::<Vec<&str>>();

    parse_site_len_wrong(&text, parse_error_message.clone())?;

    let text = text[1].split("title=\"").collect::<Vec<&str>>();

    parse_site_len_wrong(&text, parse_error_message.clone())?;

    let text = text[1].split("\"").collect::<Vec<&str>>();

    parse_site_len_wrong(&text, parse_error_message.clone())?;

    let get_number = Regex::new(r"(\d+(\.\d+)?)").unwrap();

    let chapter = get_number
        .captures(text[0])
        .and_then(|captures| captures.get(1))
        .map(|m| m.as_str())
        .ok_or(CheckError::Parse(
            "Couldn't find the chapter string".to_string(),
        ))?;

    let chapter = chapter.parse::<f32>().map_err(|err| {
        CheckError::Parse(format!("Couldn't parse float mangareader {err} {}", url))
    })?;

    Ok(chapter)
}
