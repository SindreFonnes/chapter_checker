use futures::{stream, StreamExt, TryStreamExt};

mod common_fn;
mod data;
mod handler;

use common_fn::{announce_new_chapter};
use data::{
    get_entries, get_latest_read_chapters, update_current_state, CurrentChapterState, Entry,
};
use handler::handle;
use chrono::{Utc};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let entries = get_entries();

    let result: Result<Vec<(String, f32)>, _> = stream::iter(entries.iter().map(handle))
        .buffer_unordered(30)
        .try_collect()
        .await;

    let result = result.unwrap();

    let mut entries_with_newest_chapter: Vec<(&Entry, f32)> = vec![];
    let mut next_state: Vec<CurrentChapterState> = vec![];

    let latest_chapter_states = get_latest_read_chapters();

    for site in &result {
        for entry in &entries {
            if site.0 == entry.url {
                entries_with_newest_chapter.push((entry, site.1));

                let mut state_missing: bool = true;

                for state in &latest_chapter_states {
                    if state.name == entry.name {
                        state_missing = false;
                        if state.last_chapter_read < site.1 {
                            announce_new_chapter(entry, &site.1, &state.last_chapter_read, &state.last_updated)
                        }
                        break;
                    }
                }

                if state_missing {
                    next_state.push(CurrentChapterState {
                        name: entry.name.clone(),
                        last_chapter_read: 0.0,
                        last_updated: format!("{}", Utc::now()),
                    })
                }

                break;
            }
        }
    }

    update_current_state(next_state)
}
