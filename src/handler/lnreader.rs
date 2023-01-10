use super::CheckError;
use crate::common_fn::{get_chapter_numbers_from_string, SEPERATOR};

pub fn check(text: String, url: String) -> Result<f32, CheckError> {
	let text: Vec<&str> = text.split("<div class=\"novels-detail-right-in-left\">Latest Chapters:</div>").collect();

    let text: Vec<&str> = text[1].split("<div class=\"novels-detail-right-in-right\">").collect();

    let text: Vec<_> = text[1].split("</a>").collect();

	let text: Vec<_> = text[0].split("class=\"box\">").collect();

    let text = text[1];

    let text = get_chapter_numbers_from_string(text)?;

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
