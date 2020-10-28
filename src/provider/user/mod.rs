mod csv;
pub use self::csv::CSV;

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
