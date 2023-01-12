use super::CheckError;
use crate::common_fn::{get_chapter_regex_from_string};

pub fn check(text: String, _: String) -> Result<f32, CheckError> {
    let text: Vec<&str> = text.split("<span>New Chapter</span>").collect();

    let text: Vec<_> = text[1].split("</span>").collect();
	
    let text = text[0];
	
    let text = get_chapter_regex_from_string(text)?;

    let chapter = text.parse::<f32>().unwrap();

    Ok(chapter)
}
