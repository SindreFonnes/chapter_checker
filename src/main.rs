use futures::{stream, StreamExt};
use std::collections::HashMap;

mod common_fn;
mod data;
mod handler;

use chrono::Utc;
use common_fn::announce_new_chapter;
use data::{
    get_entries, get_latest_read_chapters, update_current_state, CurrentChapterState, Entry,
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

    for site in results {
        if let Ok((entry, chapter)) = site {
            state
                .entry(entry.name.clone())
                .and_modify(|current| {
                    if current.last_chapter_read < chapter {
                        announce_new_chapter(
                            &entry,
                            &chapter,
                            &current.last_chapter_read,
                            &current.last_updated,
                        );
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

    for arg in args {
        if arg == "read" {
            update_current_state(state);
            break;
        }
    }
}
