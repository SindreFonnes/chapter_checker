use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SiteDomain {
    Manganato,
    Asura,
    Wuxiax,
    Flamescans,
    Lnreader,
    Manhwafreak,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub domain: SiteDomain,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub a_url: String,
    pub urls: Vec<Site>,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct CurrentChapterState {
    pub last_chapter_read: f32,
    pub last_updated: String,
}

pub struct ReleaseStruct {
    pub entry: Entry,
    pub newest_chapter: f32,
    pub last_read_chapter: f32,
    pub last_updated: String,
}
