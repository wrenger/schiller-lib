use gdnative::prelude::*;
use chrono::Datelike;

use crate::api;

/// The Date wrapper "class"
///
/// It provides two date formats:
/// - The iso date: %Y-%m-%d like 2001-07-08
/// - The locale date, which is based on the language of the OS (en: %m/%d/%y)
#[derive(NativeClass, Debug)]
#[inherit(Reference)]
pub struct Date {
    date: chrono::NaiveDate,
}

#[methods]
impl Date {
    fn new(_owner: &Reference) -> Self {
        Date {
            date: chrono::Local::today().naive_local(),
        }
    }

    /// The iso date: %Y-%m-%d like 2001-07-08
    #[export]
    fn get_iso(&self, _owner: &Reference) -> GodotString {
        self.date.format("%F").to_string().into()
    }
    #[export]
    fn set_iso(&mut self, _owner: &Reference, date: GodotString) -> api::Result<()> {
        self.date = chrono::NaiveDate::parse_from_str(&date.to_string(), "%F")?;
        Ok(())
    }

    /// The locale date, which is based on the language of the OS (en: %m/%d/%y)
    #[export]
    fn get_local(&self, _owner: &Reference) -> GodotString {
        self.date.format("%x").to_string().into()
    }
    #[export]
    fn set_local(&mut self, _owner: &Reference, date: GodotString) -> api::Result<()> {
        self.date = chrono::NaiveDate::parse_from_str(&date.to_string(), "%x")?;
        Ok(())
    }

    #[export]
    fn get_year(&self, _owner: &Reference) -> i64 {
        self.date.year() as _
    }
    #[export]
    fn get_month(&self, _owner: &Reference) -> i64 {
        self.date.month() as _
    }
    #[export]
    fn get_day(&self, _owner: &Reference) -> i64 {
        self.date.day() as _
    }

    /// Return the number of days until today.
    #[export]
    fn days_until_today(&self, _owner: &Reference) -> i64 {
        (chrono::Local::today().naive_local() - self.date).num_days()
    }
}

impl From<chrono::ParseError> for api::Error {
    fn from(e: chrono::ParseError) -> api::Error {
        godot_print!("chrono::ParseError: {}", e);
        api::Error::LogicError
    }
}
