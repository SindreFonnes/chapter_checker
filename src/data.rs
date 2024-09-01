use crate::structs_and_types::{CurrentChapterState, Entry, ReleaseStruct, Site};
use chrono::Utc;
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, remove_file, write};
use std::io::Write;

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

fn get_current_site_url_state_full_path() -> String {
    format!("{}/{}", get_state_location(), CURRENT_SITE_STATE_FILE)
}

fn check_if_current_site_url_state_exists() -> bool {
    std::path::Path::new(&get_current_site_url_state_full_path()).is_file()
}

fn read_current_site_url_state() -> HashMap<String, Entry> {
    let file = read_to_string(get_current_site_url_state_full_path())
        .expect("Failed to read from current_site_state file");

    let state: Vec<(String, Entry)> =
        from_str(&file).expect("Failed to parse current_site_state file");

    let mut result: HashMap<String, Entry> = HashMap::new();

    for entry in state {
        result.insert(entry.0, entry.1);
    }

    result
}

fn update_site_url_state(new_state: &HashMap<String, Entry>) {
    let mut next_state: Vec<(&str, &Entry)> = vec![];

    for (name, state) in new_state {
        next_state.push((name, state));
    }

    next_state.sort_by(|a, b| a.0.cmp(&b.0));

    write(
        get_current_site_url_state_full_path(),
        serde_json::to_string_pretty(&next_state).expect("Failed to stringify current_site_state"),
    )
    .expect("Failed to write to current_site_state file");
}

fn process_site_urls_and_update_if_match_is_found(
    old_url: &str,
    new_url: &str,
    entry: &Entry,
) -> Vec<Site> {
    let mut next_site_state: Vec<Site> = vec![];

    for site in &entry.urls {
        if site.url == old_url {
            next_site_state.push(Site {
                url: new_url.to_owned().clone(),
                domain: site.domain.clone(),
            });
        }
        next_site_state.push(Site {
            url: site.url.clone(),
            domain: site.domain.clone(),
        })
    }

    next_site_state
}

pub(crate) fn change_a_site_url_state(old_url: &str, new_url: &str) {
    let current_state = read_current_site_url_state();

    let mut next_state: HashMap<String, Entry> = HashMap::new();

    for (name, mut entry) in current_state {
        let sites = process_site_urls_and_update_if_match_is_found(old_url, new_url, &entry);

        entry.urls = sites;

        next_state.insert(name, entry);
    }

    update_site_url_state(&next_state);
}

pub(crate) fn get_current_site_url_state() -> HashMap<String, Entry> {
    if !check_if_current_site_url_state_exists() {
        create_dir_all(get_state_location())
            .expect("Could not create .app_data/chapter_checker folder");

        std::fs::File::create(get_current_site_url_state_full_path())
            .expect("Could not create site state file");

        let init_site_state: HashMap<String, Entry> = {
            let mut result: HashMap<String, Entry> = HashMap::new();
            for entry in get_init_entries() {
                let key = entry.name.clone();
                result.insert(key, entry);
            }
            result
        };

        update_site_url_state(&init_site_state);

        return init_site_state;
    }

    let state = read_current_site_url_state();

    let state_keys = state.keys();
    let init_entries = get_init_entries();

    if state_keys.len() < init_entries.len() {
        let mut next_state: HashMap<String, Entry> = state.clone();

        for entry in init_entries {
            if !next_state.contains_key(&entry.name) {
                next_state.insert(entry.name.clone(), entry);
            }
        }

        update_site_url_state(&next_state);

        return next_state;
    }

    state
}

pub fn wipe_site_state_file() {
    remove_file(get_current_site_url_state_full_path())
        .expect("Failed to wipe current_site_state file");
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

pub fn get_current_read_chapter_state() -> HashMap<String, CurrentChapterState> {
    if !check_if_current_read_chapter_state_exists() {
        create_dir_all(get_state_location())
            .expect("Failed to create .app_data/chapter_checker folder");

        let mut state_file = std::fs::File::create(get_current_read_chapter_state_full_path())
            .expect("Couldn't create current_chapter_state file");

        state_file
            .write_all("[]".as_bytes())
            .expect("Failed to write to state file");
    }

    let file = read_to_string(get_current_read_chapter_state_full_path())
        .expect("Failed to read current_chapter_state file");

    let entries: Vec<(String, CurrentChapterState)> =
        from_str(&file).expect("Failed to parse current_chapter_state file");

    let mut state: HashMap<String, CurrentChapterState> = HashMap::new();

    for (name, chapter_state) in entries {
        state.insert(name, chapter_state);
    }

    state
}

pub fn update_read_chapter_state(new_releases: &Vec<ReleaseStruct>) {
    if new_releases.is_empty() {
        return;
    }

    let mut state = get_current_read_chapter_state();

    for release in new_releases {
        let name = release.entry.name.clone();

        state
            .entry(name)
            .and_modify(|current| {
                if current.last_chapter_read < release.newest_chapter {
                    current.last_chapter_read = release.newest_chapter;
                    current.last_updated = format!("{}", Utc::now());
                }
            })
            .or_insert(CurrentChapterState {
                last_chapter_read: 0.0,
                last_updated: format!("{}", Utc::now()),
            });
    }

    let mut next_state: Vec<(String, CurrentChapterState)> = vec![];

    for (name, state) in state {
        next_state.push((name, state));
    }

    next_state.sort_by(|a, b| a.0.cmp(&b.0));

    write(
        get_current_read_chapter_state_full_path(),
        serde_json::to_string_pretty(&next_state).unwrap(),
    )
    .unwrap();
}

pub fn wipe_stored_read_chapter_data() {
    remove_file(get_current_read_chapter_state_full_path()).unwrap();
}
