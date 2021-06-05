pub mod marc21;

#[derive(Debug, Default, PartialEq, gdnative::ToVariant, gdnative::FromVariant)]
pub struct BookData {
    title: String,
    authors: Vec<String>,
    publisher: String,
    costs: f64,
}
