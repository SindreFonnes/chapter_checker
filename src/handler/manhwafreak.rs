use super::CheckError;
use crate::common_fn::{filter_non_number_chars_from_string, parse_site_len_wrong};
use crate::structs_and_types::SEPERATOR;

pub fn check(text: String, url: String) -> Result<f32, CheckError> {
    let text: Vec<&str> = text.split("<h2>Lastest Chapters</h2>").collect();

    let parse_error_message = format!("Error parsing manhwafreak: {}", url);

    parse_site_len_wrong(&text, parse_error_message.clone())?;

    let text: Vec<&str> = text[1].split("<div class=\"chapter-info\">").collect();

    parse_site_len_wrong(&text, parse_error_message.clone())?;

    let text: Vec<_> = text[1].split("</p>").collect();

    parse_site_len_wrong(&text, parse_error_message.clone())?;

    let text = text[0];

    let text = filter_non_number_chars_from_string(text);

    let chapter = match text.parse::<f32>() {
        Ok(float) => float,
        Err(e) => {
            println!("{SEPERATOR}");
            println!("Could not find / parse chapters for:");
            println!("{}", url);
            println!("{SEPERATOR}");
            return Err(CheckError::FloatErr(e));
        }
    };

    Ok(chapter)
}
