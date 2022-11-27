pub mod manganato;
pub mod asura;

#[derive(Debug)]
pub enum CheckError {
    Request(reqwest::Error),
    Parse(String),
}