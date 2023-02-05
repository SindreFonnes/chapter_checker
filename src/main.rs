mod common_fn;
mod data;
mod handler;
mod structs_and_types;

use common_fn::{check_and_anounce_chapter, check_for_chapter_updates};
use data::{update_read_chapter_state, wipe_stored_read_chapter_data};

use crate::data::wipe_site_state_file;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let new_releases= check_for_chapter_updates().await;

    check_and_anounce_chapter(&new_releases);

    for arg in args {
        if arg == "write" {
            println!("Updating readlist...");
            update_read_chapter_state(&new_releases);
        }
        if arg == "wipe" {
            println!("Wiping app-data...");
            wipe_stored_read_chapter_data();
        }
        if arg == "wipeSite" {
            println!("Wiping site url state...");
            wipe_site_state_file();
        }
    }
}
