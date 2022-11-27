mod common_fn;
mod data;
mod manga_handlers;

use data::{get_data, Data, Site};
use manga_handlers::manganato::check_manga_site;

#[tokio::main]
async fn main() {
    let data = get_data();

    let manganato_sites = data.mangas.get("manganato").unwrap();
    //let asura_sites = data.mangas.get("asura").unwrap();

    let mut futures = Vec::with_capacity(manganato_sites.len());

    for site in manganato_sites {
        futures.push(check_manga_site(&site));
    }

    let result = futures::future::try_join_all(futures).await;

    println!("{:#?}", result.unwrap());
}
