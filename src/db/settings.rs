use serde::{Deserialize, Serialize};
use tracing::error;

use crate::error::{Error, Result};
use crate::mail::account_is_valid;
use std::iter::FromIterator;

use super::{DBIter, Database, FromRow};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Settings {
    // Borrowing
    pub borrowing_duration: i64,
    // DNB
    pub dnb_token: String,
    // Mail
    pub mail_last_reminder: String,
    pub mail_from: String,
    pub mail_host: String,
    pub mail_password: String,
    // Mail Templates
    pub mail_info_subject: String,
    pub mail_info_content: String,
    pub mail_overdue_subject: String,
    pub mail_overdue_content: String,
    pub mail_overdue2_subject: String,
    pub mail_overdue2_content: String,
}

impl Settings {
    pub fn set(&mut self, key: String, value: String) {
        match key.as_str() {
            "version" => {}
            "borrowing.duration" => {
                self.borrowing_duration = value.parse().unwrap_or(self.borrowing_duration);
            }
            "dnb.token" => self.dnb_token = value,
            "mail.lastReminder" => self.mail_last_reminder = value,
            "mail.from" => self.mail_from = value,
            "mail.host" => self.mail_host = value,
            "mail.password" => self.mail_password = value,
            "mail.info.subject" => self.mail_info_subject = value,
            "mail.info.content" => self.mail_info_content = value,
            "mail.overdue.subject" => self.mail_overdue_subject = value,
            "mail.overdue.content" => self.mail_overdue_content = value,
            "mail.overdue2.subject" => self.mail_overdue2_subject = value,
            "mail.overdue2.content" => self.mail_overdue2_content = value,
            _ => error!("Unknown option: {key} = {value}"),
        };
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            borrowing_duration: 28,
            dnb_token: String::new(),
            mail_last_reminder: String::new(),
            mail_from: String::new(),
            mail_host: String::new(),
            mail_password: String::new(),
            mail_info_subject: String::new(),
            mail_info_content: String::new(),
            mail_overdue_subject: String::new(),
            mail_overdue_content: String::new(),
            mail_overdue2_subject: String::new(),
            mail_overdue2_content: String::new(),
        }
    }
}

impl FromIterator<(String, String)> for Settings {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut settings = Settings::default();
        for (key, value) in iter {
            settings.set(key, value);
        }
        settings
    }
}

impl FromRow for (String, String) {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok((row.get(0)?, row.get(1)?))
    }
}

pub fn update(db: &Database, settings: &Settings) -> Result<()> {
    let mail_from = settings.mail_from.trim();
    if !mail_from.is_empty() && !account_is_valid(mail_from) {
        return Err(Error::InvalidUser);
    }

    db.con.execute(
        "replace into sbv_meta values \
        ('borrowing.duration', ?), \
        ('dnb.token', ?), \
        ('mail.lastReminder', ?), \
        ('mail.from', ?), \
        ('mail.host', ?), \
        ('mail.password', ?), \
        ('mail.info.subject', ?), \
        ('mail.info.content', ?), \
        ('mail.overdue.subject', ?), \
        ('mail.overdue.content', ?), \
        ('mail.overdue2.subject', ?), \
        ('mail.overdue2.content', ?)",
        rusqlite::params![
            settings.borrowing_duration,
            settings.dnb_token.trim(),
            settings.mail_last_reminder.trim(),
            mail_from,
            settings.mail_host.trim(),
            settings.mail_password.trim(),
            settings.mail_info_subject.trim(),
            settings.mail_info_content.trim(),
            settings.mail_overdue_subject.trim(),
            settings.mail_overdue_content.trim(),
            settings.mail_overdue2_subject.trim(),
            settings.mail_overdue2_content.trim(),
        ],
    )?;
    Ok(())
}

pub fn fetch(db: &Database) -> Result<Settings> {
    let mut stmt = db.con.prepare("select key, value from sbv_meta")?;
    let rows = stmt.query([])?;
    DBIter::new(rows).collect()
}
