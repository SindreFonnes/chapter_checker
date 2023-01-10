use super::CheckError;
use regex::Regex;

pub fn check(text: String, url: String) -> Result<f32, CheckError> {
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
        .map_err(|err| {
            CheckError::Parse(format!(
                "Couldn't parse float manganato {err} {}",
                url
            ))
        })?;

    Ok(chapter)
}
