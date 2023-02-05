use super::CheckError;
use crate::structs_and_types::SEPERATOR;
use regex::Regex;

pub fn check(text: String, url: String) -> Result<f32, CheckError> {
    let text: Vec<&str> = text
        .split("<div class=\"novels-detail-right-in-left\">Latest Chapters:</div>")
        .collect();

    let text: Vec<&str> = text[1]
        .split("<div class=\"novels-detail-right-in-right\">")
        .collect();

    let text: Vec<_> = text[1].split("</a>").collect();

    let text: Vec<_> = text[0].split("class=\"box\">").collect();

    let text = text[1];

    let float_regex = Regex::new(r"[-+]?[0-9]*\.?[0-9]+").unwrap();

    let text: String = float_regex.find_iter(text).map(|m| m.as_str()).collect();

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
