use chrono::Datelike;
use gdnative::prelude::*;

use crate::api;

/// The Date wrapper "class"
///
/// It provides two date formats:
/// - The iso date: %Y-%m-%d like 2001-07-08
/// - The locale date, which is based on the language of the OS (en: %m/%d/%y)
#[derive(NativeClass, Debug)]
#[register_with(Date::register)]
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

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .add_property("year")
            .with_getter(Date::get_year)
            .with_setter(Date::set_year)
            .done();
        builder
            .add_property("month")
            .with_getter(Date::get_month)
            .with_setter(Date::set_month)
            .done();
        builder
            .add_property("day")
            .with_getter(Date::get_day)
            .with_setter(Date::set_day)
            .done();
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

    fn get_year(&self, _owner: TRef<Reference>) -> i64 {
        self.date.year() as _
    }
    fn set_year(&mut self, _owner: TRef<Reference>, year: i64) {
        if let Some(new) = self.date.with_year(year as _) {
            self.date = new;
        }
    }

    fn get_month(&self, _owner: TRef<Reference>) -> i64 {
        self.date.month() as _
    }
    fn set_month(&mut self, _owner: TRef<Reference>, month: i64) {
        if let Some(new) = self.date.with_month(month as _) {
            self.date = new;
        }
    }

    fn get_day(&self, _owner: TRef<Reference>) -> i64 {
        self.date.day() as _
    }
    fn set_day(&mut self, _owner: TRef<Reference>, day: i64) {
        if let Some(new) = self.date.with_day(day as _) {
            self.date = new;
        }
    }

    /// Return the number of days until today.
    #[export]
    fn days_until_today(&self, _owner: &Reference) -> i64 {
        (chrono::Local::today().naive_local() - self.date).num_days()
    }
}

impl From<chrono::ParseError> for api::Error {
    fn from(e: chrono::ParseError) -> api::Error {
        error!("chrono::ParseError: {e}");
        api::Error::Logic
    }
}
