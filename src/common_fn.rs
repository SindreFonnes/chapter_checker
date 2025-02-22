use std::collections::HashMap;

use chrono::Utc;
use futures::{stream, StreamExt};
//use rand::distributions::{Distribution, Uniform};
use regex::Regex;
use reqwest::{RequestBuilder, Response};

use crate::data::{
    change_a_site_url_state, get_current_read_chapter_state, get_current_site_url_state,
    get_init_entries,
};
use crate::structs_and_types::{Entry, ReleaseStruct};
use crate::{
    handler::{handle, CheckError},
    structs_and_types::CurrentChapterState,
};

pub(crate) fn parse_site_len_wrong(
    arr: &Vec<&str>,
    error_string: String,
) -> Result<(), CheckError> {
    if arr.len() < 2 {
        return Err(CheckError::Parse(error_string));
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
            println!("\nError for site: {}", site);
            println!("{:?}", err);
            return Err(err);
        }
    }
    .error_for_status()
    {
        Ok(resp) => resp,
        Err(err) => {
            println!("\nError status for site: {}", site);
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
            .unwrap_or_else(|| panic!("Failed to unwrap new url to {}", site));
        println!("{} changed url to {:?}", site, &new_url);

        let new_url = new_url.to_str().unwrap_or_else(|_| {
            panic!(
                "Failed to parse new_url to string for {}, {:?}",
                site, new_url
            )
        });

        let response = match get_site_respone(new_url).await {
            Ok(resp) => resp,
            Err(err) => return Err(err),
        };

        change_a_site_url_state(site, new_url);

        return response.text().await;
    }

    response.text().await
}

fn get_entries() -> Vec<Entry> {
    let state = {
        let mut result: HashMap<String, Entry> = HashMap::new();
        for entry in get_init_entries() {
            let key = entry.name.clone();
            result.insert(key, entry);
        }
        result
    };

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

    for site in results.into_iter().flatten() {
        let entry = site.0;
        let chapter = site.1;

        let old_entry_state = state.get(&entry.name);

        match old_entry_state {
            Some(previous_state) => {
                if previous_state.last_chapter_read < chapter {
                    new_releases.push(ReleaseStruct {
                        entry: entry.clone(),
                        newest_chapter: chapter,
                        last_read_chapter: previous_state.last_chapter_read,
                        last_updated: previous_state.last_updated.clone(),
                    })
                }
            }
            None => new_releases.push(ReleaseStruct {
                entry: entry.clone(),
                newest_chapter: chapter,
                last_read_chapter: 0.0,
                last_updated: format!("{}", Utc::now()),
            }),
        }
    }

    new_releases
}
