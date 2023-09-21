use crate::common_fn::parse_site_len_wrong;

use super::CheckError;
use regex::Regex;

pub fn check(text: String, url: String) -> Result<f32, CheckError> {
    let text: Vec<&str> = text
        .lines()
        .filter(|segment| segment.contains("chapter"))
        .filter(|segment| segment.contains("</a>") && segment.len() < 2000)
        .collect();

    let parse_error_message = format!("Error parsing manganato: {}", url);

    parse_site_len_wrong(&text, parse_error_message.clone())?;

    let text = text[0].split(">").skip(1).next().unwrap().to_lowercase();

    let text: Vec<_> = text.split("chapter").collect();

    parse_site_len_wrong(&text, parse_error_message.clone())?;

    let get_number = Regex::new(r"([0-9])\w+").unwrap();

    let chapter = get_number
        .captures(text[1])
        .and_then(|captures| captures.get(0))
        .map(|m| m.as_str())
        .ok_or(CheckError::Parse(
            "Couldn't find the chapter string".to_string(),
        ))?;

    let chapter = chapter.parse::<f32>().map_err(|err| {
        CheckError::Parse(format!("Couldn't parse float manganato {err} {}", url))
    })?;

    Ok(chapter)
}
