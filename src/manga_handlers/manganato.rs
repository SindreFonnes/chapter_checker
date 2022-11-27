use super::CheckError;
use crate::common_fn::get_site_as_string;
use crate::data::Site;
use regex::Regex;

pub async fn check(site: &Site) -> Result<f32, CheckError> {
    let text = get_site_as_string(&site.url)
        .await
        .map_err(|err| CheckError::Request((err)))?;

    let text: Vec<&str> = text
        .lines()
        .filter(|segment| segment.contains("chapter"))
        .filter(|segment| segment.contains("</a>") && segment.len() < 2000)
        .collect();

    let text = text[0].split(">").skip(1).next().unwrap().to_lowercase();

    let text: Vec<_> = text.split("chapter").collect();

    let re = Regex::new(r"[^\d.]").unwrap();

    let final_text = re
        .replace_all(text[1], "")
        .to_string()
        .parse::<f32>()
        .map_err(|err| CheckError::Parse(format!("Couldn't parse float manganato {err} {}", site.name)))?;

    println!("{:#?}", final_text);
    Ok(final_text)
}
