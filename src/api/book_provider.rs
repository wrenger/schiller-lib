use std::cell::RefCell;

use gdnative::prelude::*;

use crate::request;

/// The BookProvider wrapper "class"
#[derive(NativeClass)]
#[inherit(Reference)]
pub struct BookProvider {
    provider: Option<Box<RefCell<dyn request::BookProvider>>>,
}

#[methods]
impl BookProvider {
    fn new(_owner: &Reference) -> Self {
        BookProvider { provider: None }
    }

    #[export]
    fn get_providers(&self, _owner: &Reference) -> Vec<request::BookProviderType> {
        request::BookProviderType::values()
    }

    #[export]
    fn set_provider(&mut self, _owner: &Reference, provider: request::BookProviderType) {
        self.provider = Some(Box::new(RefCell::new(request::book(provider))))
    }

    #[export]
    fn get_options(&self, _owner: &Reference) -> Vec<String> {
        if let Some(provider) = &self.provider {
            provider.borrow().options()
        } else {
            Vec::new()
        }
    }

    #[export]
    fn configure(&self, _owner: &Reference, key: String, value: String) -> request::Result<()> {
        if let Some(provider) = &self.provider {
            provider.borrow_mut().configure(&key, &value)
        } else {
            Err(request::Error::InvalidConfig)
        }
    }

    #[export]
    fn request(&self, _owner: &Reference, isbn: String) -> request::Result<request::BookData> {
        if let Some(isbn) = crate::isbn::parse(&isbn) {
            if let Some(provider) = &self.provider {
                provider.borrow().request(&isbn)
            } else {
                Err(request::Error::InvalidConfig)
            }
        } else {
            Err(request::Error::InvalidInput)
        }
    }
}
