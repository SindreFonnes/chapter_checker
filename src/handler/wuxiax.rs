use crate::common_fn::get_numbers_with_regex_capture;

use super::CheckError;

pub fn check(text: String, _: String) -> Result<f32, CheckError> {
    let text: Vec<&str> = text.split("<strong>Latest").collect();

    let text: Vec<_> = text[1].split("</a>").collect();

    let text = text[0].to_lowercase();

    let text: Vec<_> = text.split("\">chapter").collect();

    let text = text[1];

    let text = get_numbers_with_regex_capture(text)?;

    let chapter = text.parse::<f32>().unwrap();

    Ok(chapter)
}
