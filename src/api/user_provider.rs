use std::cell::RefCell;

use gdnative::prelude::*;

use crate::provider::{self, Provider, UserData, UserProviderType};

/// The UserProvider wrapper "class"
#[derive(NativeClass)]
#[inherit(Reference)]
pub struct UserProvider {
    provider: Option<Box<RefCell<dyn Provider<UserData>>>>,
}

#[methods]
impl UserProvider {
    fn new(_owner: &Reference) -> Self {
        UserProvider { provider: None }
    }

    #[export]
    fn get_providers(&self, _owner: &Reference) -> Vec<UserProviderType> {
        UserProviderType::values()
    }

    #[export]
    fn set_provider(&mut self, _owner: &Reference, provider: UserProviderType) {
        self.provider = Some(Box::new(RefCell::new(provider::user(provider))))
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
    fn configure(&self, _owner: &Reference, key: String, value: String) -> provider::Result<()> {
        if let Some(provider) = &self.provider {
            provider.borrow_mut().configure(&key, &value)
        } else {
            Err(provider::Error::InvalidConfig)
        }
    }

    #[export]
    fn request(&self, _owner: &Reference, account: String) -> provider::Result<UserData> {
        if let Some(provider) = &self.provider {
            provider.borrow().request(account.trim())
        } else {
            Err(provider::Error::InvalidConfig)
        }
    }

    #[export]
    fn bulk_request(&self, _owner: &Reference, accounts: Vec<String>) -> provider::Result<Vec<UserData>> {
        if let Some(provider) = &self.provider {
            let accounts: Vec<&str> = accounts.iter().map(|a| a.trim()).collect();
            provider.borrow().bulk_request(&accounts)
        } else {
            Err(provider::Error::InvalidConfig)
        }
    }
}
