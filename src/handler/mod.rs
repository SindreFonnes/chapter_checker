pub mod asura;
pub mod manganato;

use crate::data::{Entry, Site};

#[derive(Debug)]
pub enum CheckError {
    Request(reqwest::Error),
    Parse(String),
}

pub async fn handle(entry: &Entry) -> Result<(String, f32), CheckError> {
    match entry.site {
        Site::Manganato => manganato::check(entry).await,
        Site::Asura => asura::check(entry).await,
        Site::Professor => Ok(("Professor".to_string(), 100.0)),
    }
}
