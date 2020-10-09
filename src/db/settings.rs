use crate::api;
use std::collections::HashMap;

use super::{DBIter, ReadStmt};

const SETTINGS_FETCH: &str = r#"
select key, value from sbv_meta
"#;

const SETTINGS_UPDATE: &str = r#"
replace into sbv_meta values
('borrowing.duration', ?),
('user.path', ?),
('user.delimiter', ?),
('dnb.token', ?),
('mail.lastReminder', ?),
('mail.from', ?),
('mail.host', ?),
('mail.password', ?),
('mail.info.subject', ?),
('mail.info.content', ?),
('mail.overdue.subject', ?),
('mail.overdue.content', ?),
('mail.overdue2.subject', ?),
('mail.overdue2.content', ?)
"#;

#[derive(Debug, Clone, gdnative::ToVariant, gdnative::FromVariant)]
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

impl Settings {
    fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> api::Result<Self> {
        let mut settings = Settings::default();
        for (key, value) in iter {
            match key.as_str() {
                "version" => {},
                "borrowing.duration" => {
                    settings.borrowing_duration =
                        value.parse().unwrap_or(settings.borrowing_duration)
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
                _ => gdnative::godot_error!("Unknown option: {} = {}", key, value),
            }
        }
        Ok(settings)
    }
}

impl ReadStmt for (String, String) {
    type Error = api::Error;
    fn read(
        stmt: &sqlite::Statement,
        _columns: &HashMap<String, usize>,
    ) -> Result<Self, Self::Error> {
        Ok((stmt.read(0)?, stmt.read(1)?))
    }
}

pub trait DatabaseSettings {
    fn db(&self) -> &sqlite::Connection;

    fn settings_update(&self, settings: &Settings) -> api::Result<()> {
        let mut stmt = self.db().prepare(SETTINGS_UPDATE)?;
        stmt.bind(1, settings.borrowing_duration)?;
        stmt.bind(2, settings.user_path.as_str())?;
        stmt.bind(3, settings.user_delimiter.as_str())?;
        stmt.bind(4, settings.dnb_token.as_str())?;
        stmt.bind(5, settings.mail_last_reminder.as_str())?;
        stmt.bind(6, settings.mail_from.as_str())?;
        stmt.bind(7, settings.mail_host.as_str())?;
        stmt.bind(8, settings.mail_password.as_str())?;
        stmt.bind(9, settings.mail_info_subject.as_str())?;
        stmt.bind(10, settings.mail_info_content.as_str())?;
        stmt.bind(11, settings.mail_overdue_subject.as_str())?;
        stmt.bind(12, settings.mail_overdue_content.as_str())?;
        stmt.bind(13, settings.mail_overdue2_subject.as_str())?;
        stmt.bind(14, settings.mail_overdue2_content.as_str())?;

        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        Ok(())
    }

    fn settings_fetch(&self) -> api::Result<Settings> {
        let stmt = self.db().prepare(SETTINGS_FETCH)?;
        Settings::from_iter(DBIter::new(stmt))
    }
}
