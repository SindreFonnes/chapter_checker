pub mod asura;
pub mod manganato;
pub mod professor;
pub mod flamescans;

use core::fmt;
use std::num::ParseFloatError;

use crate::data::{Entry, Site};

#[derive(Debug)]
pub enum CheckError {
    Request(reqwest::Error),
    Parse(String),
    FloatErr(ParseFloatError)
}

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Checkerror")
    }
}

pub async fn handle(entry: &Entry) -> Result<(Entry, f32), CheckError> {
    match entry.site {
        Site::Manganato => manganato::check(entry).await,
        Site::Asura => asura::check(entry).await,
        Site::Professor => professor::check(entry).await,
        Site::Flamescans => flamescans::check(entry).await,
    }
}
