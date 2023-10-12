use std::path::Path;

use crate::db::User;
use crate::error::{Error, Result};

const ACCOUNT: usize = 0;
const FORENAME: usize = 1;
const SURNAME: usize = 2;
const ROLE: usize = 3;

/// Load all users and roles from the userfile.
pub fn load_roles(file: &Path, delimiter: u8) -> Result<Vec<(String, String)>> {
    if !delimiter.is_ascii() {
        return Err(Error::Arguments);
    }

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .from_path(file)?;

    let mut pairs = Vec::new();
    for result in reader.records() {
        let record = result?;
        pairs.push((
            record.get(ACCOUNT).ok_or(Error::InvalidFormat)?.into(),
            record.get(ROLE).ok_or(Error::InvalidFormat)?.into(),
        ))
    }
    Ok(pairs)
}

/// Search for a specific user
pub fn search(file: &Path, delimiter: u8, account: &str) -> Result<User> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .from_path(file)?;

    for record in reader.records() {
        let record = record?;
        if record.get(ACCOUNT) == Some(account) {
            return Ok(User {
                account: account.into(),
                forename: record.get(FORENAME).ok_or(Error::InvalidFormat)?.into(),
                surname: record.get(SURNAME).ok_or(Error::InvalidFormat)?.into(),
                role: record.get(ROLE).ok_or(Error::InvalidFormat)?.into(),
                may_borrow: true,
            });
        }
    }
    Err(Error::NothingFound)
}
