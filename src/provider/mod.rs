pub mod marc21;

use gdnative::derive::{FromVariant, ToVariant};

#[derive(Debug, Default, PartialEq, ToVariant, FromVariant)]
pub struct BookData {
    title: String,
    authors: Vec<String>,
    publisher: String,
    costs: f64,
}
