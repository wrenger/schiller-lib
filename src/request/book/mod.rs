mod dnb;
use crate::request;

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

pub fn book(provider: BookProviderType) -> impl BookProvider {
    match provider {
        BookProviderType::DNB => dnb::DNB::new(),
    }
}

pub trait BookProvider {
    fn options(&self) -> Vec<String>;
    fn configure(&mut self, key: &str, value: &str) -> request::Result<()>;
    fn request(&self, isbn: &str) -> request::Result<BookData>;
}
