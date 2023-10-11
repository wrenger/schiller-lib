use serde::Serialize;

pub mod marc21;


#[derive(Debug, Default, PartialEq, Serialize)]
pub struct BookData {
    title: String,
    authors: Vec<String>,
    publisher: String,
    costs: f64,
}
