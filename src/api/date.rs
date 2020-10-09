use gdnative::prelude::*;

use crate::api;

/// The Date wrapper "class"
#[derive(NativeClass, Debug)]
#[inherit(Reference)]
pub struct Date {
    pub date: chrono::NaiveDate,
}

#[methods]
impl Date {
    fn new(_owner: &Reference) -> Self {
        Date {
            date: chrono::Local::today().naive_local(),
        }
    }
    #[export]
    fn get_iso(&self, _owner: &Reference) -> GodotString {
        self.date.format("%F").to_string().into()
    }
    #[export]
    fn set_iso(&mut self, _owner: &Reference, date: GodotString) -> api::Result<()> {
        self.date = chrono::NaiveDate::parse_from_str(&date.to_string(), "%F")?;
        Ok(())
    }

    #[export]
    fn get_local(&self, _owner: TRef<Reference>) -> GodotString {
        self.date.format("%x").to_string().into()
    }
    #[export]
    fn set_local(&mut self, _owner: &Reference, date: GodotString) -> api::Result<()> {
        self.date = chrono::NaiveDate::parse_from_str(&date.to_string(), "%x")?;
        Ok(())
    }

    #[export]
    fn days_since_today(&self, _owner: &Reference) -> i64 {
        (chrono::Local::today().naive_local() - self.date).num_days()
    }
}
