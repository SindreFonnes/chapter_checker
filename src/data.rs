use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Site {
    Manganato,
    Asura,
    Professor
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(rename = "type")]
    pub kind: String,
    pub site: Site,
    pub name: String,
    pub url: String,
}

pub fn get_entries() -> Vec<Entry> {
    let file = std::fs::read_to_string("./data/data.json").unwrap();
    let entries = serde_json::from_str(&file).unwrap();
    entries
}
