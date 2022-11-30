use crate::data::Entry;

pub async fn get_site_as_string(site: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(site).await?.error_for_status()?;
    response.text().await
}

const SEPERATOR: &str = "----------------";

pub fn announce_new_chapter(
    entry: &Entry,
    newest_chapter: &f32,
    last_read_chapter: &f32,
    last_updated: &String,
) {
    println!("{SEPERATOR}");
    println!("Name : {}", entry.name);
    println!("Site : {:?}", entry.site);
    println!("Type : {}", entry.kind);
    println!("Url  : {}", entry.url);
    if entry.a_url != "".to_owned() {
        println!("AUrl : {}", entry.a_url);
    }
    println!("");
    println!("The newest chapter is : {}", newest_chapter);
    println!("Last read chapter was : {}", last_read_chapter);
    println!("It was last updated   : {}", last_updated);
    println!("{SEPERATOR}");
}
