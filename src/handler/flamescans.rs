use super::CheckError;
use crate::common_fn::{get_chapter_regex_from_string, parse_site_len_wrong};

pub fn check(text: String, url: String) -> Result<f32, CheckError> {
    let text: Vec<&str> = text.split("<span>New Chapter</span>").collect();

    let parse_error_message = format!("Error parsing Flamescans: {}", url);

    parse_site_len_wrong(&text, parse_error_message.clone())?;

    let text: Vec<_> = text[1].split("</span>").collect();

    parse_site_len_wrong(&text, parse_error_message.clone())?;
	
    let text = text[0];
	
    let text = get_chapter_regex_from_string(text)?;

    let chapter = text.parse::<f32>().unwrap();

    Ok(chapter)
}
