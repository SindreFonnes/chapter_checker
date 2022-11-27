use super::CheckError;
use crate::common_fn::get_site_as_string;
use crate::data::Site;
use regex::Regex;

pub async fn check(site: &Site) -> Result<f32, CheckError> {
    let text = get_site_as_string(&site.url)
        .await
        .map_err(|err| CheckError::Request(err))?;

    let text: Vec<&str> = text.split("chapterlist").collect();

    let text: Vec<_> = text[1].split("</li>").collect();

    let re = Regex::new(r"Chapter [0-9]*").unwrap();

    let text = re.find(text[0]);

    let final_text = text
        .ok_or(CheckError::Parse(
            format!("Couldn't find the chapter string asura {}", site.name),
        ))?
        .as_str()
        .parse::<f32>()
        .unwrap();

    println!("{:#?}", final_text);
    Ok(final_text)
}
