use crate::common_fn::{get_numbers_with_regex_capture, parse_site_len_wrong};

use super::CheckError;

pub fn check(text: String, url: String) -> Result<f32, CheckError> {
    let text: Vec<&str> = text.split("<strong>Latest").collect();

    let parse_error_message = format!("Error parsing Wuxiax: {}", url);

    parse_site_len_wrong(&text, parse_error_message.clone())?;

    let text: Vec<_> = text[1].split("</a>").collect();

    parse_site_len_wrong(&text, parse_error_message.clone())?;

    let text = text[0].to_lowercase();


    let text: Vec<_> = text.split("\">chapter").collect();

    parse_site_len_wrong(&text, parse_error_message.clone())?;

    let text = text[1];

    let text = get_numbers_with_regex_capture(text)?;

    let chapter = text.parse::<f32>().unwrap();

    Ok(chapter)
}
