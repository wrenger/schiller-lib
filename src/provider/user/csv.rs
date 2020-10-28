use std::collections::HashSet;

use crate::api;
use crate::provider::user::UserData;

#[derive(Debug)]
pub struct CSV {
    pub path: String,
    pub delimiter: u8,
    pub has_headers: bool,
    pub column_account: usize,
    pub column_forename: usize,
    pub column_surname: usize,
    pub column_role: usize,
}

impl Default for CSV {
    fn default() -> CSV {
        CSV {
            path: String::new(),
            delimiter: b',',
            has_headers: false,
            column_account: 0,
            column_forename: 1,
            column_surname: 2,
            column_role: 3,
        }
    }
}

impl CSV {
    pub fn request(&self, account: &str) -> api::Result<UserData> {
        let reader = csv::ReaderBuilder::new()
            .has_headers(self.has_headers)
            .delimiter(self.delimiter)
            .from_path(&self.path)?;

        for record in reader.into_records() {
            let record = record?;
            if record
                .get(self.column_account)
                .ok_or(api::Error::InvalidArguments)?
                .trim()
                == account
            {
                return Ok(UserData::from(
                    account,
                    record
                        .get(self.column_forename)
                        .ok_or(api::Error::InvalidArguments)?
                        .trim(),
                    record
                        .get(self.column_surname)
                        .ok_or(api::Error::InvalidArguments)?
                        .trim(),
                    record
                        .get(self.column_role)
                        .ok_or(api::Error::InvalidArguments)?
                        .trim(),
                ));
            }
        }

        Err(api::Error::NothingFound)
    }

    pub fn bulk_request(&self, accounts: &[&str]) -> api::Result<Vec<UserData>> {
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
                .ok_or(api::Error::InvalidArguments)?
                .trim();
            if accounts.contains(&account) {
                result.push(UserData::from(
                    account,
                    record
                        .get(self.column_forename)
                        .ok_or(api::Error::InvalidArguments)?
                        .trim(),
                    record
                        .get(self.column_surname)
                        .ok_or(api::Error::InvalidArguments)?
                        .trim(),
                    record
                        .get(self.column_role)
                        .ok_or(api::Error::InvalidArguments)?
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
        let csv_provider = CSV {
            path: "test/csv/users.csv".into(),
            ..CSV::default()
        };
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
        let csv_provider = CSV {
            path: "test/csv/users.csv".into(),
            ..CSV::default()
        };
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
