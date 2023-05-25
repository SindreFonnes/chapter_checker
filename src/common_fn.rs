use std::collections::HashMap;

use chrono::Utc;
use futures::{stream, StreamExt};
//use rand::distributions::{Distribution, Uniform};
use regex::Regex;
use reqwest::{RequestBuilder, Response};

use crate::data::{
    change_a_site_url_state, get_current_read_chapter_state, get_current_site_url_state,
};
use crate::structs_and_types::{Entry, ReleaseStruct};
use crate::{
    handler::{handle, CheckError},
    structs_and_types::CurrentChapterState,
};

pub(crate) fn parse_site_len_wrong (arr: &Vec<&str>, error_string: String) -> Result<(), CheckError> {
    if arr.len() < 2 {
        return Err(CheckError::Parse(error_string))
    }
    Ok(())
}

pub(crate) fn get_chapter_regex_from_string(input: &str) -> Result<&str, CheckError> {
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

pub(crate) fn get_numbers_with_regex_capture(input: &str) -> Result<&str, CheckError> {
    let re = Regex::new(r" ([0-9]*)").unwrap();

    let text = re
        .captures(input)
        .and_then(|captures| captures.get(1))
        .map(|m| m.as_str())
        .ok_or(CheckError::Parse("Couldn't find regex numbers".to_string()))?;

    Ok(text)
}

pub(crate) fn filter_non_number_chars_from_string(input: &str) -> String {
    let float_regex = Regex::new(r"[-+]?[0-9]*\.?[0-9]+").unwrap();

    let text: String = float_regex.find_iter(input).map(|m| m.as_str()).collect();

    text
}

//const USER_AGENT_LIST: [&str; 3] = [
//    "Mozilla/5.0 (iPad; CPU OS 12_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148",
//    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.83 Safari/537.36",
//    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.51 Safari/537.36"
//];
//
//fn get_user_agent() -> &'static str {
//    let mut rng = rand::thread_rng();
//    let range = Uniform::from(0..3);
//    let random_index = range.sample(&mut rng);
//    println!("{}", random_index);
//    USER_AGENT_LIST[random_index]
//}

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
        .header("Accept-Encoding", "gzip, deflate")
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

pub(crate) async fn get_site_as_string(site: &str) -> Result<String, reqwest::Error> {
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

        let new_url = new_url.to_str().expect(
            format!(
                "Failed to parse new_url to string for {}, {:?}",
                site, new_url
            )
            .as_str(),
        );

        let response = match get_site_respone(&new_url).await {
            Ok(resp) => resp,
            Err(err) => return Err(err),
        };

        change_a_site_url_state(site, new_url);

        return response.text().await;
    }

    response.text().await
}

fn get_entries() -> Vec<Entry> {
    let state = get_current_site_url_state();
    state.into_values().collect()
}

pub async fn check_for_chapter_updates() -> Vec<ReleaseStruct> {
    let entries = get_entries();

    let results: Vec<Result<(Entry, f32), _>> = stream::iter(entries.iter().map(handle))
        .buffer_unordered(30)
        .collect()
        .await;

    let state: HashMap<String, CurrentChapterState> = get_current_read_chapter_state();

    let mut new_releases: Vec<ReleaseStruct> = vec![];

    for site in results {
        if let Ok((entry, chapter)) = site {
            let old_entry_state = state.get(&entry.name);

            match old_entry_state {
                Some(previous_state) => {
                    if previous_state.last_chapter_read < chapter {
                        new_releases.push(ReleaseStruct {
                            entry: entry.clone(),
                            newest_chapter: chapter.clone(),
                            last_read_chapter: previous_state.last_chapter_read.clone(),
                            last_updated: previous_state.last_updated.clone(),
                        })
                    }
                }
                None => new_releases.push(ReleaseStruct {
                    entry: entry.clone(),
                    newest_chapter: chapter.clone(),
                    last_read_chapter: 0.0,
                    last_updated: format!("{}", Utc::now()),
                }),
            }
        }
    }

    new_releases
}
