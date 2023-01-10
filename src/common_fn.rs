use reqwest::{RequestBuilder, Response};

use crate::data::Entry;
use crate::handler::CheckError;
use rand::distributions::{Distribution, Uniform};
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

const USER_AGENT_LIST: [&str; 3] = [
    "Mozilla/5.0 (iPad; CPU OS 12_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.83 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.51 Safari/537.36"
];

fn get_user_agent() -> &'static str {
    let mut rng = rand::thread_rng();
    let range = Uniform::from(0..3);
    let random_index = range.sample(&mut rng);
    println!("{}", random_index);
    USER_AGENT_LIST[random_index]
}

//use reqwest::{
//    header::{ACCEPT, CACHE_CONTROL, CONNECTION, USER_AGENT},
//    RequestBuilder, Response,
//};
//.header(USER_AGENT, get_user_agent())
//.header(CONNECTION, "keep-alive")
//.header(CACHE_CONTROL, "max-age=0")
//.header(
//    "sec-ch-ua",
//    "\" Not A;Brand\";v=\"99\", \"Chromium\";v=\"99\", \"Google Chrome\";v=\"99\"",
//)
//.header("sec-ch-ua-mobile", "?0")
//.header("sec-ch-ua-platform", "macOS")
//.header("Upgrade-Insecure-Requests", "1")
//.header(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
//.header("Sec-Fetch-Site", "none")
//.header("Sec-Fetch-Mode", "navigate")
//.header("Sec-Fetch-User", "?1")
//.header("Sec-Fetch-Dest", "document")
//.header("Accept-Language", "en-GB,en-US;q=0.9,en;q=0.8")
//.header("Accept-Encoding", "gzip, deflate, br")

fn add_client_headers(request_builder: RequestBuilder) -> RequestBuilder {
    request_builder
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv,98.0) Gecko/20100101 Firefox/98.0",
        )
        .header(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        )
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Connection", "keep-alive")
        .header("Upgrade-Insecure-Requests", "1")
        .header("Sec-Fetch-Dest", "document")
        .header("Sec-Fetch-Mode", "navigate")
        .header("Sec-Fetch-Site", "none")
        .header("Sec-Fetch-User", "?1")
        .header("Cache-Control", "max-age=0")
    //.header("Accept-Encoding", "gzip, deflate")
}

async fn get_site_respone(site: &str) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = match match add_client_headers(client.get(site)).send().await {
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

pub const SEPERATOR: &str = "----------------";

pub fn announce_new_chapter(
    entry: &Entry,
    newest_chapter: &f32,
    last_read_chapter: &f32,
    last_updated: &String,
) {
    println!("{SEPERATOR}");
    println!("Name : {}", entry.name);
    println!("Type : {}", entry.kind);
    println!("Urls : {:?}", entry.urls);
    if entry.a_url != "".to_owned() {
        println!("AUrl : {}", entry.a_url);
    }
    println!("");
    println!("The newest chapter is : {}", newest_chapter);
    println!("Last read chapter was : {}", last_read_chapter);
    println!("It was last updated   : {}", last_updated);
    println!("{SEPERATOR}");
}
