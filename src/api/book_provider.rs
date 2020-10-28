use gdnative::prelude::*;

use crate::api;
use crate::provider::book::{BookData, DNB};

/// The BookDNBProvider wrapper "class"
#[derive(NativeClass)]
#[inherit(Reference)]
#[register_with(Self::register)]
pub struct BookDNBProvider {
    provider: DNB,
}

#[methods]
impl BookDNBProvider {
    fn new(_owner: &Reference) -> Self {
        BookDNBProvider {
            provider: DNB::default(),
        }
    }

    /// Perform a request to the DNB and fetch the metadata for the given isbn.
    #[export]
    fn request(&self, _owner: &Reference, isbn: String) -> api::Result<BookData> {
        if let Some(isbn) = crate::isbn::parse(&isbn) {
            self.provider.request(&isbn)
        } else {
            Err(api::Error::InvalidArguments)
        }
    }

    // Properties

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .add_property::<GodotString>("token")
            .with_getter(Self::get_token)
            .with_setter(Self::set_token)
            .done();
    }

    fn get_token(&self, _owner: TRef<Reference>) -> GodotString {
        GodotString::from_str(&self.provider.token)
    }

    fn set_token(&mut self, _owner: TRef<Reference>, token: GodotString) {
        self.provider.token = token.to_string();
    }
}
