use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs::{create_dir_all, read_to_string, write, remove_file};
use std::io::Write;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Site {
    Manganato,
    Asura,
    Professor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename = "type")]
    pub kind: String,
    pub site: Site,
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct CurrentChapterState {
    pub last_chapter_read: f32,
    pub last_updated: String,
}

pub fn get_entries() -> Vec<Entry> {
    let text = include_str!("../data/data.json");
    let entries = from_str(text).unwrap();
    entries
}

const CURRENT_STATE_FILE_NAME: &str = "current_chapter_state.json";

fn get_current_state_location() -> String {
    format!(
        "{}/.app_data/chapter_checker",
        home::home_dir().unwrap().display()
    )
}

fn get_current_state_full_path() -> String {
    format!(
        "{}/{}",
        get_current_state_location(),
        CURRENT_STATE_FILE_NAME
    )
}

fn check_if_current_state_exists() -> bool {
    std::path::Path::new(&get_current_state_full_path()).is_file()
}

pub fn get_latest_read_chapters() -> HashMap<String, CurrentChapterState> {
    if !check_if_current_state_exists() {
        create_dir_all(get_current_state_location()).unwrap();
        let mut state_file =
            std::fs::File::create(get_current_state_full_path()).expect("Couldn't create file");
        state_file
            .write_all("{}".as_bytes())
            .expect("Failed to write to state file");
    }

    let file = read_to_string(get_current_state_full_path()).unwrap();
    let entries = from_str(&file).unwrap();
    entries
}

pub fn update_current_state(new_state: &HashMap<String, CurrentChapterState>) {
    write(
        get_current_state_full_path(),
        serde_json::to_string_pretty(&new_state).unwrap(),
    )
    .unwrap();
}

pub fn wipe_stored_data () {
    remove_file(get_current_state_full_path()).unwrap();
}
