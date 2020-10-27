use std::collections::HashSet;
use std::path::PathBuf;

use crate::provider::{self, Provider, UserData};

#[derive(Debug)]
pub struct CSV {
    path: PathBuf,
    delimiter: u8,
    has_headers: bool,
    column_account: usize,
    column_forename: usize,
    column_surname: usize,
    column_role: usize,
}

impl Default for CSV {
    fn default() -> CSV {
        CSV {
            path: PathBuf::new(),
            delimiter: b',',
            has_headers: false,
            column_account: 0,
            column_forename: 1,
            column_surname: 2,
            column_role: 3,
        }
    }
}

impl Provider<UserData> for CSV {
    fn configure(&mut self, key: &str, value: &str) -> provider::Result<()> {
        let value = value.trim();
        match key {
            "path" => {
                if !value.is_empty() {
                    self.path = PathBuf::from(value)
                } else {
                    return Err(provider::Error::InvalidConfig);
                }
            }
            "delimiter" => {
                if value.chars().count() == 1 && value.is_ascii() {
                    self.delimiter =
                        value.chars().next().ok_or(provider::Error::InvalidConfig)? as u32 as u8
                } else {
                    return Err(provider::Error::InvalidConfig);
                }
            }
            "has_headers" => {
                self.has_headers = value.parse().map_err(|_| provider::Error::InvalidConfig)?
            }
            "column_account" => {
                self.column_account = value.parse().map_err(|_| provider::Error::InvalidConfig)?
            }
            "column_forename" => {
                self.column_forename = value.parse().map_err(|_| provider::Error::InvalidConfig)?
            }
            "column_surname" => {
                self.column_surname = value.parse().map_err(|_| provider::Error::InvalidConfig)?
            }
            "column_role" => {
                self.column_role = value.parse().map_err(|_| provider::Error::InvalidConfig)?
            }
            _ => return Err(provider::Error::InvalidConfig),
        }
        Ok(())
    }

    fn options(&self) -> Vec<String> {
        vec![
            "path".into(),
            "delimiter".into(),
            "has_headers".into(),
            "column_account".into(),
            "column_forename".into(),
            "column_surname".into(),
            "column_role".into(),
        ]
    }

    fn request(&self, account: &str) -> provider::Result<UserData> {
        let reader = csv::ReaderBuilder::new()
            .has_headers(self.has_headers)
            .delimiter(self.delimiter)
            .from_path(&self.path)?;

        for record in reader.into_records() {
            let record = record?;
            if record
                .get(self.column_account)
                .ok_or(provider::Error::InvalidConfig)?
                .trim()
                == account
            {
                return Ok(UserData::from(
                    account,
                    record
                        .get(self.column_forename)
                        .ok_or(provider::Error::InvalidConfig)?
                        .trim(),
                    record
                        .get(self.column_surname)
                        .ok_or(provider::Error::InvalidConfig)?
                        .trim(),
                    record
                        .get(self.column_role)
                        .ok_or(provider::Error::InvalidConfig)?
                        .trim(),
                ));
            }
        }

        Err(provider::Error::NothingFound)
    }

    fn bulk_request(&self, accounts: &[&str]) -> provider::Result<Vec<UserData>> {
        let reader = csv::ReaderBuilder::new()
            .has_headers(self.has_headers)
            .delimiter(self.delimiter)
            .from_path(&self.path)?;

        let accounts: HashSet<&&str> = accounts.iter().collect();
        let mut result = Vec::new();

        for record in reader.into_records() {
            let record = record?;
            let account = record
                .get(self.column_account)
                .ok_or(provider::Error::InvalidConfig)?
                .trim();
            if accounts.contains(&account) {
                result.push(UserData::from(
                    account,
                    record
                        .get(self.column_forename)
                        .ok_or(provider::Error::InvalidConfig)?
                        .trim(),
                    record
                        .get(self.column_surname)
                        .ok_or(provider::Error::InvalidConfig)?
                        .trim(),
                    record
                        .get(self.column_role)
                        .ok_or(provider::Error::InvalidConfig)?
                        .trim(),
                ));
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_csv() {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path("test/csv/empty.csv")
            .unwrap();
        assert!(reader.into_records().next().is_none());

        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path("test/csv/one_line.csv")
            .unwrap();
        let rows: Vec<csv::StringRecord> = reader.into_records().map(|r| r.unwrap()).collect();
        assert_eq!(1, rows.len());
        assert_eq!(rows[0], vec!["a", "b"]);

        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path("test/csv/users.csv")
            .unwrap();
        let rows: Vec<csv::StringRecord> = reader.into_records().map(|r| r.unwrap()).collect();
        assert_eq!(100, rows.len());
        assert_eq!(rows[0], vec!["callen.lawson", "Callen", "Lawson", "Person"]);
        assert_eq!(rows[99], vec!["safah.scott", "Safah", "Scott", "Person"]);
    }

    #[test]
    fn request_single() {
        let mut csv_provider = CSV::default();
        csv_provider
            .configure("path", "test/csv/users.csv")
            .unwrap();
        assert_eq!(
            csv_provider.request("callen.lawson").unwrap(),
            UserData::from("callen.lawson", "Callen", "Lawson", "Person")
        );
        assert_eq!(
            csv_provider.request("charlotte.penn").unwrap(),
            UserData::from("charlotte.penn", "Charlotte", "Penn", "Person")
        );
        assert_eq!(
            csv_provider.request("safah.scott").unwrap(),
            UserData::from("safah.scott", "Safah", "Scott", "Person")
        );
    }

    #[test]
    fn request_multiple() {
        let mut csv_provider = CSV::default();
        csv_provider
            .configure("path", "test/csv/users.csv")
            .unwrap();
        let result = csv_provider
            .bulk_request(&["charlotte.penn", "callen.lawson", "safah.scott"])
            .unwrap();
        assert_eq!(
            result,
            vec![
                UserData::from("callen.lawson", "Callen", "Lawson", "Person"),
                UserData::from("charlotte.penn", "Charlotte", "Penn", "Person"),
                UserData::from("safah.scott", "Safah", "Scott", "Person"),
            ]
        );
    }
}
