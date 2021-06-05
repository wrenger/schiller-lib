use gdnative::prelude::*;

use crate::api;
use crate::provider::{marc21, BookData};

/// The Marc21Parser wrapper "class"
#[derive(NativeClass)]
#[inherit(Reference)]
pub struct Marc21 {}

#[methods]
impl Marc21 {
    fn new(_owner: &Reference) -> Self {
        Marc21 {}
    }

    /// Perform a request to the DNB and fetch the metadata for the given isbn.
    #[export]
    fn parse(&self, _owner: &Reference, isbn: String, response: String) -> api::Result<BookData> {
        if let Ok(isbn) = crate::isbn::parse(&isbn) {
            marc21::parse(&response, &isbn)
        } else {
            Err(api::Error::InvalidISBN)
        }
    }
}
