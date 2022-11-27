use futures::{stream, StreamExt, TryStreamExt};

mod common_fn;
mod data;
mod handler;

use data::get_entries;
use handler::handle;

#[tokio::main]
async fn main() {
    let entries = get_entries();

    let result: Result<Vec<(String, f32)>, _> = stream::iter(entries.iter().map(handle))
        .buffer_unordered(30)
        .try_collect()
        .await;

    println!("{:#?}", result.unwrap());
}
