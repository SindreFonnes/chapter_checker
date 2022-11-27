use futures::{stream, StreamExt, TryStreamExt};

mod common_fn;
mod data;
mod manga_handlers;

use data::{get_data};
use manga_handlers::manganato::check;

#[tokio::main]
async fn main() {
    let data = get_data();

    let manganato_sites = data.mangas.get("manganato").unwrap();
    let asura_sites = data.mangas.get("asura").unwrap();

    let stream = stream::iter(manganato_sites.iter().map(check));

    let result: Result<Vec<f32>, _> = stream.buffer_unordered(20).try_collect().await;

    // let result = futures::future::try_join_all(futures).await;

    println!("{:#?}", result.unwrap());
}
