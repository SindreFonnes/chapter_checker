use futures::{stream, StreamExt};
use std::collections::HashMap;

mod common_fn;
mod data;
mod handler;

use chrono::Utc;
use common_fn::announce_new_chapter;
use data::{
    get_entries, get_latest_read_chapters, update_current_state, CurrentChapterState, Entry, wipe_stored_data
};
use handler::handle;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let entries = get_entries();

    let results: Vec<Result<(Entry, f32), _>> = stream::iter(entries.iter().map(handle))
        .buffer_unordered(30)
        .collect()
        .await;

    let mut state: HashMap<String, CurrentChapterState> = get_latest_read_chapters();
    let mut new_releases: Vec<(Entry, f32, f32, String)> = vec![];

    for site in results {
        if let Ok((entry, chapter)) = site {
            state
                .entry(entry.name.clone())
                .and_modify(|current| {
                    if current.last_chapter_read < chapter {
                        new_releases.push((
                            entry.clone(),
                            chapter.clone(),
                            current.last_chapter_read.clone(),
                            current.last_updated.clone(),
                        ));
                        current.last_chapter_read = chapter;
                        current.last_updated = format!("{}", Utc::now());
                    }
                })
                .or_insert(CurrentChapterState {
                    last_chapter_read: 0.0,
                    last_updated: format!("{}", Utc::now()),
                });
        }
    }

    if new_releases.len() > 0 {
        println!("There are new chapters for:");
        for entry in new_releases {
            announce_new_chapter(&entry.0, &entry.1, &entry.2, &entry.3);
        }
    } else {
        println!("There are no new chapters...")
    }

    for arg in args {
        if arg == "write" {
            println!("Updating readlist...");
            update_current_state(&state);
        }
        if arg == "wipe" {
            println!("Wiping app-data...");
            wipe_stored_data();
        }
    }
}
