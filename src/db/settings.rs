use crate::api;
use std::iter::FromIterator;

use super::{DBIter, Database, FromRow};

use gdnative::derive::{FromVariant, ToVariant};

const SETTINGS_FETCH: &str = "\
    select key, value from sbv_meta \
";

const SETTINGS_UPDATE: &str = "\
    replace into sbv_meta values \
    ('borrowing.duration', ?), \
    ('user.path', ?), \
    ('user.delimiter', ?), \
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
    ('mail.overdue2.content', ?) \
";

#[derive(Debug, PartialEq, Clone, ToVariant, FromVariant)]
pub struct Settings {
    // Borrowing
    pub borrowing_duration: i64,
    // User
    pub user_path: String,
    pub user_delimiter: String,
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

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            borrowing_duration: 28,
            user_path: String::new(),
            user_delimiter: ",".into(),
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
            match key.as_str() {
                "version" => {}
                "borrowing.duration" => {
                    settings.borrowing_duration =
                        value.parse().unwrap_or(settings.borrowing_duration);
                }
                "user.path" => settings.user_path = value,
                "user.delimiter" => settings.user_delimiter = value,
                "dnb.token" => settings.dnb_token = value,
                "mail.lastReminder" => settings.mail_last_reminder = value,
                "mail.from" => settings.mail_from = value,
                "mail.host" => settings.mail_host = value,
                "mail.password" => settings.mail_password = value,
                "mail.info.subject" => settings.mail_info_subject = value,
                "mail.info.content" => settings.mail_info_content = value,
                "mail.overdue.subject" => settings.mail_overdue_subject = value,
                "mail.overdue.content" => settings.mail_overdue_content = value,
                "mail.overdue2.subject" => settings.mail_overdue2_subject = value,
                "mail.overdue2.content" => settings.mail_overdue2_content = value,
                _ => error!("Unknown option: {key} = {value}"),
            }
        }
        settings
    }
}

impl FromRow for (String, String) {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok((row.get(0)?, row.get(1)?))
    }
}

pub fn update(db: &Database, settings: &Settings) -> api::Result<()> {
    db.con.execute(
        SETTINGS_UPDATE,
        rusqlite::params![
            settings.borrowing_duration,
            settings.user_path.trim(),
            settings.user_delimiter.trim(),
            settings.dnb_token.trim(),
            settings.mail_last_reminder.trim(),
            settings.mail_from.trim(),
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

pub fn fetch(db: &Database) -> api::Result<Settings> {
    let mut stmt = db.con.prepare(SETTINGS_FETCH)?;
    let rows = stmt.query([])?;
    DBIter::new(rows).collect()
}
