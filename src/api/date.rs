use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{Datelike, NaiveDate};
use gdnative::api::OS;
use gdnative::prelude::*;

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
    /// The year number in the calendar date.
    #[allow(dead_code)]
    #[property(get = "Self::get_year", set = "Self::set_year")]
    year: Property<i64>,
    /// The month number starting from 1.
    /// The value ranges from 1 to 12.
    #[allow(dead_code)]
    #[property(get = "Self::get_month", set = "Self::set_month")]
    month: Property<i64>,
    /// The day of month starting from 1.
    /// The value ranges from 1 to 31. (The last day of month differs by months.)
    #[allow(dead_code)]
    #[property(get = "Self::get_day", set = "Self::set_day")]
    day: Property<i64>,
}

#[methods]
impl Date {
    fn new(_owner: &Reference) -> Self {
        Date {
            date: chrono::Local::now().date_naive(),
            year: Property::default(),
            month: Property::default(),
            day: Property::default(),
        }
    }

    /// The iso date: %Y-%m-%d like 2001-07-20
    #[method]
    fn get_iso(&self) -> String {
        self.date.format("%F").to_string()
    }
    /// The iso date: %Y-%m-%d like 2001-07-20
    #[method]
    fn set_iso(&mut self, date: GodotString) -> api::Result<()> {
        self.date = chrono::NaiveDate::parse_from_str(&date.to_string(), "%F")?;
        Ok(())
    }

    /// The locale date, which is based on the language of the system (en: %m/%d/%y)
    #[method]
    fn get_locale(&self) -> String {
        let locale = OS::godot_singleton().get_locale().to_string();
        if let Some(res) = format_localized(self.date, "%x", &locale) {
            res
        } else {
            error!("Unknown locale {locale:?}");
            self.get_iso()
        }
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
    #[method]
    fn days_until_today(&self) -> i64 {
        (chrono::Local::now().date_naive() - self.date).num_days()
    }
}

/// Format a date according to the systems language (`locale`)
fn format_localized(date: NaiveDate, fmt: &str, locale: &str) -> Option<String> {
    if let Ok(locale) = chrono::Locale::try_from(locale) {
        let items = StrftimeItems::new_with_locale(fmt, locale);
        let res = DelayedFormat::new_with_locale(Some(date), None, items, locale);
        Some(res.to_string())
    } else {
        None
    }
}

impl From<chrono::ParseError> for api::Error {
    fn from(e: chrono::ParseError) -> api::Error {
        error!("chrono::ParseError: {e}");
        api::Error::Logic
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    #[test]
    fn dates() {
        let date = NaiveDate::from_ymd_opt(2022, 10, 20).unwrap();
        assert_eq!(
            "10/20/2022",
            super::format_localized(date, "%x", "en_US").unwrap()
        );
        assert_eq!(
            "20.10.2022",
            super::format_localized(date, "%x", "de_DE").unwrap()
        );
    }
}
