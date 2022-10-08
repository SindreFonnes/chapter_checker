use std::collections::HashMap;

use regex::Regex;
use reqwest::blocking::{get as reqwest_get, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Site {
    name: String,
    #[serde(rename = "site")]
    url: String,
    last_read_chapter: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Data {
    manga: Vec<Site>,
    light_novel: HashMap<String, Site>,
}

fn get_site_as_string(site: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest_get(site).unwrap();
    match resp.error_for_status().and_then(|response| response.text()) {
        Ok(result_text) => Ok(result_text),
        Err(reqwest_error) => Err(reqwest_error),
    }
}

fn check_manga_site(site: &Site) {
    let resp = get_site_as_string(&site.url);
    let text = match resp {
        Ok(text) => text,
        Err(error) => {
            if error.is_status() {
                println!("{:#?}", error.status())
            }
            return;
        }
    };

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
        .unwrap();

    println!("{:#?}", final_text);
}

fn main() {
    let file = std::fs::read_to_string("./src/data.json").unwrap();
    let data = serde_json::from_str::<Data>(&file).unwrap();

    println!("{:#?}", data);
}
