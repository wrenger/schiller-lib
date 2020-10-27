mod dnb;
use crate::provider;

#[derive(Debug, Default, PartialEq, gdnative::ToVariant, gdnative::FromVariant)]
pub struct BookData {
    title: String,
    authors: Vec<String>,
    publisher: String,
    costs: f64,
}

#[derive(Debug, gdnative::ToVariant, gdnative::FromVariant)]
pub enum BookProviderType {
    DNB,
}

impl BookProviderType {
    pub fn values() -> Vec<BookProviderType> {
        vec![BookProviderType::DNB]
    }
}

pub fn book(provider: BookProviderType) -> impl provider::Provider<BookData> {
    match provider {
        BookProviderType::DNB => dnb::DNB::default(),
    }
}
