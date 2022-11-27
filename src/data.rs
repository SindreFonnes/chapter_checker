use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub name: String,
    #[serde(rename = "site")]
    pub url: String,
    pub last_read_chapter: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub mangas: HashMap<String, Vec<Site>>,
    pub light_novel: HashMap<String, Site>,
}

pub fn get_data() -> Data {
    let file = std::fs::read_to_string("./data/data.json").unwrap();
    let data = serde_json::from_str::<Data>(&file).unwrap();
    data
}
