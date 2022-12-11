use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, remove_file, write};
use std::io::Write;

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
    pub a_url: String,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct CurrentChapterState {
    pub last_chapter_read: f32,
    pub last_updated: String,
}

fn get_init_entries() -> Vec<Entry> {
    let text = include_str!("../data/data.json");
    let entries = from_str(text).unwrap();
    entries
}

const STATE_LOCATION_FOLDER: &str = ".app_data/chapter_checker";

fn get_state_location() -> String {
    format!(
        "{}/{}",
        home::home_dir().unwrap().display(),
        STATE_LOCATION_FOLDER
    )
}

const CURRENT_SITE_STATE_FILE: &str = "current_site_url_state.json";

fn get_current_site_state_full_path() -> String {
    format!("{}/{}", get_state_location(), CURRENT_SITE_STATE_FILE)
}

fn check_if_current_site_state_exists() -> bool {
    std::path::Path::new(&get_current_site_state_full_path()).is_file()
}

pub fn get_current_site_state() -> HashMap<String, Entry> {
    if !check_if_current_site_state_exists() {
        create_dir_all(get_state_location())
            .expect("Could not create .app_data/chapter_checker folder");

        std::fs::File::create(get_current_site_state_full_path())
            .expect("Could not create site state file");

        let init_site_state: HashMap<String, Entry> = {
            let mut result: HashMap<String, Entry> = HashMap::new();
            for entry in get_init_entries() {
                let key = entry.name.clone();
                result.insert(key, entry);
            }
            result
        };

        write(
            get_current_site_state_full_path(),
            serde_json::to_string_pretty(&init_site_state).unwrap(),
        )
        .expect("Failed to write init state to current_site_state file");

        return init_site_state;
    }

    let file = read_to_string(get_current_site_state_full_path())
        .expect("Failed to read from current_site_state file");
    let state = from_str(&file).expect("Failed to parse current_site_state file");
    state
}

pub fn update_site_state(new_state: HashMap<String, Entry>) {
    write(
        get_current_site_state_full_path(),
        serde_json::to_string_pretty(&new_state).expect("Failed to stringify current_site_state"),
    )
    .expect("Failed to write to current_site_state file");
}

pub fn wipe_site_state_file() {
    remove_file(get_current_site_state_full_path())
        .expect("Failed to wipe current_site_state file");
}

pub fn get_entries() -> Vec<Entry> {
    let state = get_current_site_state();
    state.into_values().collect()
}

const CURRENT_READ_CHAPTER_STATE_FILE_NAME: &str = "current_chapter_state.json";

fn get_current_read_chapter_state_full_path() -> String {
    format!(
        "{}/{}",
        get_state_location(),
        CURRENT_READ_CHAPTER_STATE_FILE_NAME
    )
}

fn check_if_current_read_chapter_state_exists() -> bool {
    std::path::Path::new(&get_current_read_chapter_state_full_path()).is_file()
}

pub fn get_latest_read_chapters() -> HashMap<String, CurrentChapterState> {
    if !check_if_current_read_chapter_state_exists() {
        create_dir_all(get_state_location())
            .expect("Failed to create .app_data/chapter_checker folder");
        let mut state_file = std::fs::File::create(get_current_read_chapter_state_full_path())
            .expect("Couldn't create current_chapter_state file");
        state_file
            .write_all("{}".as_bytes())
            .expect("Failed to write to state file");
    }

    let file = read_to_string(get_current_read_chapter_state_full_path())
        .expect("Failed to read current_chapter_state file");
    let entries = from_str(&file).expect("Failed to parse current_chapter_state file");
    entries
}

pub fn update_read_chapter_state(new_state: &HashMap<String, CurrentChapterState>) {
    write(
        get_current_read_chapter_state_full_path(),
        serde_json::to_string_pretty(&new_state).unwrap(),
    )
    .unwrap();
}

pub fn wipe_stored_read_chapter_data() {
    remove_file(get_current_read_chapter_state_full_path()).unwrap();
}
