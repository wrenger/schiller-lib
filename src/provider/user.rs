use std::collections::HashMap;
use std::path::Path;

use crate::db::User;
use crate::db::sorted::Sorted;
use crate::error::{Error, Result};
use crate::fuzzy;

const ACCOUNT: usize = 0;
const FORENAME: usize = 1;
const SURNAME: usize = 2;
const ROLE: usize = 3;

trait StringRecordExt {
    fn get_i(&self, index: usize) -> Result<&str>;
}
impl StringRecordExt for csv::StringRecord {
    fn get_i(&self, index: usize) -> Result<&str> {
        self.get(index)
            .map(|s| s.trim())
            .ok_or(Error::InvalidFormat)
    }
}

/// Load all users and roles from the userfile.
pub fn load_roles(file: &Path, delimiter: u8) -> Result<HashMap<String, String>> {
    let mut reader = reader(file, delimiter)?;

    let mut pairs = HashMap::new();
    for result in reader.records() {
        let record = result?;
        let account = record.get_i(ACCOUNT)?;
        let role = record.get_i(ROLE)?;
        pairs
            .entry(account.to_string())
            .or_insert_with(|| role.to_string());
    }
    Ok(pairs)
}

fn parse_record(record: &csv::StringRecord) -> Result<User> {
    Ok(User {
        account: record.get_i(ACCOUNT)?.to_string(),
        forename: record.get_i(FORENAME)?.to_string(),
        surname: record.get_i(SURNAME)?.to_string(),
        role: record.get_i(ROLE)?.to_string(),
        may_borrow: true,
    })
}

#[allow(unused)]
pub fn load_all(file: &Path, delimiter: u8) -> Result<Vec<User>> {
    let mut reader = reader(file, delimiter)?;
    let mut users = Vec::new();
    for result in reader.records() {
        users.push(parse_record(&result?)?);
    }
    Ok(users)
}

/// Search for a specific user
pub fn get(file: &Path, delimiter: u8, account: &str) -> Result<User> {
    let mut reader = reader(file, delimiter)?;
    for record in reader.records() {
        let record = record?;
        if record.get(ACCOUNT) == Some(account) {
            return parse_record(&record);
        }
    }
    Err(Error::NothingFound)
}

#[allow(unused)]
pub fn search(file: &Path, delimiter: u8, search: &str, count: usize) -> Result<Vec<User>> {
    let mut reader = reader(file, delimiter)?;
    let query = search.trim();
    let mut fuzzy = (!query.is_empty()).then(|| fuzzy::Fuzzy::new(query));

    let mut results = Sorted::<(u32, User), _>::new(|a, b| {
        a.0.cmp(&b.0)
            .reverse()
            .then_with(|| a.1.account.cmp(&b.1.account))
    });
    for record in reader.records() {
        let user = parse_record(&record?)?;
        if let Some(fuzzy) = &mut fuzzy {
            let score = user.fuzzy(fuzzy);
            if score > 0 {
                results.push((score, user));
            }
        } else {
            results.push((0, user));
        }
    }
    Ok(results
        .into_iter()
        .take(count)
        .map(|(_, user)| user)
        .collect())
}

fn reader(file: &Path, delimiter: u8) -> Result<csv::Reader<std::fs::File>> {
    if !delimiter.is_ascii() {
        return Err(Error::Arguments);
    }
    Ok(csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .from_path(file)?)
}
