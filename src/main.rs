use chapter_checker::check_for_chapter_updates;
use chapter_checker::data::{
    update_read_chapter_state, wipe_site_state_file, wipe_stored_read_chapter_data,
};
use chapter_checker::structs_and_types::{ReleaseStruct, Entry};

const SEPERATOR: &str = "----------------";

fn announce_new_chapter(
    entry: &Entry,
    newest_chapter: &f32,
    last_read_chapter: &f32,
    last_updated: &String,
) {
    println!("{SEPERATOR}");
    println!("Name : {}", entry.name);
    println!("Type : {}", entry.kind);
    println!("Urls : {:?}", entry.urls);
    if entry.a_url != "".to_owned() {
        println!("AUrl : {}", entry.a_url);
    }
    println!("");
    println!("The newest chapter is : {}", newest_chapter);
    println!("Last read chapter was : {}", last_read_chapter);
    println!("It was last updated   : {}", last_updated);
    println!("{SEPERATOR}");
}

fn check_and_anounce_chapter(new_releases: &Vec<ReleaseStruct>) {
    if new_releases.len() > 0 {
        println!("There are new chapters for:");
        for entry in new_releases {
            announce_new_chapter(
                &entry.entry,
                &entry.newest_chapter,
                &entry.last_read_chapter,
                &entry.last_updated,
            );
        }
        return;
    }

    println!("There are no new chapters...")
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let new_releases = check_for_chapter_updates().await;

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
