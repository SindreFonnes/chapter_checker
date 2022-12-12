use reqwest::Response;

use crate::data::Entry;
use crate::handler::CheckError;
use regex::Regex;

pub fn get_chapter_numbers_from_string(input: &str) -> Result<&str, CheckError> {
    let re = Regex::new(r"Chapter ([0-9,.]*)").unwrap();

    let text = re
        .captures(input)
        .and_then(|captures| captures.get(1))
        .map(|m| m.as_str())
        .ok_or(CheckError::Parse(
            "Couldn't find the chapter string".to_string(),
        ))?;

    Ok(text)
}

async fn get_site_respone(site: &str) -> Result<Response, reqwest::Error> {
    let response = match match reqwest::get(site).await {
        Ok(resp) => resp,
        Err(err) => {
            println!("Error for site: {}", site);
            println!("{:?}", err);
            return Err(err);
        }
    }
    .error_for_status()
    {
        Ok(resp) => resp,
        Err(err) => {
            println!("Error for site: {}", site);
            println!("{:?}", err);
            return Err(err);
        }
    };
    Ok(response)
}

pub async fn get_site_as_string(site: &str) -> Result<String, reqwest::Error> {
    let response = match get_site_respone(site).await {
        Ok(resp) => resp,
        Err(err) => return Err(err),
    };

    if response.status().as_u16() == 302 {
        println!("{} changed url", site);

        let headers = response.headers();
        let new_url = headers
            .get("location")
            .expect(format!("Failed to unwrap new url to {}", site).as_str());
        println!("{} changed url to {:?}", site, &new_url);

        let response = match get_site_respone(
            &new_url.to_str().expect(
                format!(
                    "Failed to parse new_url to string for {}, {:?}",
                    site, new_url
                )
                .as_str(),
            ),
        )
        .await
        {
            Ok(resp) => resp,
            Err(err) => return Err(err),
        };

        return response.text().await;
    }

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
