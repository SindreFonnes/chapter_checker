use super::CheckError;
use crate::common_fn::{get_site_as_string, get_chapter_numbers_from_string};
use crate::data::Entry;

pub async fn check(entry: &Entry) -> Result<(Entry, f32), CheckError> {
    let text = get_site_as_string(&entry.url)
        .await
        .map_err(|err| CheckError::Request(err))?;

    let text: Vec<&str> = text.split("chapterlist").collect();

    let text: Vec<_> = text[1].split("</li>").collect();

    let text = text[0];

    let text = get_chapter_numbers_from_string(text)?;

    let chapter = text
        .parse::<f32>()
        .unwrap();

    Ok((entry.clone(), chapter))
}
