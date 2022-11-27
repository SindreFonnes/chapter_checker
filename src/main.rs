use futures::{stream, StreamExt, TryStreamExt};

mod common_fn;
mod data;
mod handler;

use data::{get_entries};
use handler::handle;

#[tokio::main]
async fn main() {
    let entries = get_entries();

    let stream = stream::iter(entries.iter().map(handle));

    let result: Result<Vec<(String, f32)>, _> = stream.buffer_unordered(20).try_collect().await;

    // let result = futures::future::try_join_all(futures).await;

    println!("{:#?}", result.unwrap());
}
