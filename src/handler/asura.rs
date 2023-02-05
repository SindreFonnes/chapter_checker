use super::CheckError;
use crate::common_fn::{get_chapter_regex_from_string};
use crate::structs_and_types::SEPERATOR;

pub fn check(text: String, url: String) -> Result<f32, CheckError> {
    let text: Vec<&str> = text.split("chapterlist").collect();

    let text: Vec<_> = text[1].split("</li>").collect();

    let text = text[0];

    let text = get_chapter_regex_from_string(text)?;

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
