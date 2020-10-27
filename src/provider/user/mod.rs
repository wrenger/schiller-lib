mod csv;

use crate::provider::Provider;

#[derive(Debug, Default, PartialEq, gdnative::ToVariant, gdnative::FromVariant)]
pub struct UserData {
    account: String,
    forename: String,
    surname: String,
    role: String,
}

impl UserData {
    fn from(account: &str, forename: &str, surname: &str, role: &str) -> UserData {
        UserData {
            account: account.into(),
            forename: forename.into(),
            surname: surname.into(),
            role: role.into(),
        }
    }
}

#[derive(Debug, gdnative::ToVariant, gdnative::FromVariant)]
pub enum UserProviderType {
    CSV,
}

impl UserProviderType {
    pub fn values() -> Vec<UserProviderType> {
        vec![UserProviderType::CSV]
    }
}

pub fn user(provider: UserProviderType) -> impl Provider<UserData> {
    match provider {
        UserProviderType::CSV => csv::CSV::default(),
    }
}
