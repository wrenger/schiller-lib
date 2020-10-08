use gdnative::prelude::*;

use crate::db::DBSettings;

/// The Settings wrapper "class"
#[derive(NativeClass, Debug, Default)]
#[inherit(Reference)]
pub struct Settings {
    #[property]
    pub version: GodotString,
    // Borrowing
    #[property]
    pub borrowing_duration: i64,
    // User
    #[property]
    pub user_path: GodotString,
    #[property]
    pub user_delimiter: GodotString,
    // DNB
    #[property]
    pub dnb_token: GodotString,
    // Mail
    #[property]
    pub mail_last_reminder: GodotString,
    #[property]
    pub mail_from: GodotString,
    #[property]
    pub mail_host: GodotString,
    #[property]
    pub mail_password: GodotString,
    // Mail Templates
    #[property]
    pub mail_info_subject: GodotString,
    #[property]
    pub mail_info_content: GodotString,
    #[property]
    pub mail_overdue_subject: GodotString,
    #[property]
    pub mail_overdue_content: GodotString,
    #[property]
    pub mail_overdue2_subject: GodotString,
    #[property]
    pub mail_overdue2_content: GodotString,
}

#[methods]
impl Settings {
    fn new(_owner: &Reference) -> Self {
        let mut settings = Settings::default();
        settings.fill(DBSettings::default());
        settings
    }

    pub fn db_instance(settings: DBSettings) -> Instance<Settings, Unique> {
        let instance = Settings::new_instance();
        instance.map_mut(|x, _| x.fill(settings)).unwrap();
        instance
    }

    pub fn fill(&mut self, settings: DBSettings) {
        self.version = settings.version.into();
        self.borrowing_duration = settings.borrowing_duration;
        self.user_path = settings.user_path.into();
        self.user_delimiter = settings.user_delimiter.into();
        self.dnb_token = settings.dnb_token.into();
        self.mail_last_reminder = settings.mail_last_reminder.into();
        self.mail_from = settings.mail_from.into();
        self.mail_host = settings.mail_host.into();
        self.mail_password = settings.mail_password.into();
        self.mail_info_subject = settings.mail_info_subject.into();
        self.mail_info_content = settings.mail_info_content.into();
        self.mail_overdue_subject = settings.mail_overdue_subject.into();
        self.mail_overdue_content = settings.mail_overdue_content.into();
        self.mail_overdue2_subject = settings.mail_overdue2_subject.into();
        self.mail_overdue2_content = settings.mail_overdue2_content.into();
    }

    pub fn db(&self) -> DBSettings {
        DBSettings {
            version: self.version.to_string(),
            borrowing_duration: self.borrowing_duration,
            user_path: self.user_path.to_string(),
            user_delimiter: self.user_delimiter.to_string(),
            dnb_token: self.dnb_token.to_string(),
            mail_last_reminder: self.mail_last_reminder.to_string(),
            mail_from: self.mail_from.to_string(),
            mail_host: self.mail_host.to_string(),
            mail_password: self.mail_password.to_string(),
            mail_info_subject: self.mail_info_subject.to_string(),
            mail_info_content: self.mail_info_content.to_string(),
            mail_overdue_subject: self.mail_overdue_subject.to_string(),
            mail_overdue_content: self.mail_overdue_content.to_string(),
            mail_overdue2_subject: self.mail_overdue2_subject.to_string(),
            mail_overdue2_content: self.mail_overdue2_content.to_string(),
        }
    }
}
