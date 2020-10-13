use std::fmt::{self, Display};
use std::str::FromStr;

use crate::api;

use super::{raw::DatabaseExt, DatabaseSettings, Settings};

const CREATE_TABLES: &str = r#"
create table sbv_meta (
key text primary key,
value text not null);

create table author (
name text not null,
medium text not null,
primary key (name, medium));

create table user (
account text not null primary key,
forename text not null,
surname text not null,
role text not null,
may_borrow integer not null default 1);

create table category (
id text not null primary key,
name text not null,
section text not null);

create table medium (
id text not null primary key,
isbn text not null,
title text not null,
publisher text not null,
year integer,
costs real,
note text not null,
borrowable integer not null,
category text not null,
borrower text not null default '',
deadline text not null default '',
reservation text not null default '');
"#;

const FETCH_VERSION: &str = r#"
select value from sbv_meta where key='version'
"#;

const UPDATE_VERSION: &str = r#"
replace into sbv_meta values ('version', ?)
"#;

/// Minimum supported version.
const MIN_VERSION: Version = Version::new(0, 7, 0);

type MigrationRoutine = fn(&sqlite::Connection) -> api::Result<()>;

/// Database migration routines
const PATCHES: [(Version, MigrationRoutine); 1] = [(Version::new(0, 8, 0), patch_0_8_0)];

pub trait DatabaseStructure: DatabaseSettings {
    fn structure_create(&self, version: &str) -> api::Result<()> {
        let transaction = self.db().transaction()?;
        self.db().execute(CREATE_TABLES)?;
        update_version(self.db(), version)?;
        self.settings_update(&Settings::default())?;
        transaction.commit()?;
        Ok(())
    }

    /// Applies the related migration routines if the version changed.
    /// Returns true if the database was updated.
    fn structure_migrate(&self, version: &str) -> api::Result<bool> {
        let transaction = self.db().transaction()?;
        let mut stmt = self.db().prepare(FETCH_VERSION)?;
        let old_version = if stmt.next()? == sqlite::State::Row {
            stmt.read::<String>(0)?
        } else {
            return Err(api::Error::UnsupportedProjectVersion);
        };
        gdnative::godot_print!("Start migration of {}", old_version);

        let old_version: Version = old_version.parse()?;
        let new_version: Version = version.parse()?;
        if MIN_VERSION <= old_version && old_version <= new_version {
            for (patch_version, patch) in &PATCHES {
                if old_version < *patch_version {
                    gdnative::godot_print!("Applying patch {}", patch_version);
                    patch(self.db())?;
                }
            }
            update_version(self.db(), version)?;
            transaction.commit()?;
            Ok(old_version != new_version)
        } else {
            Err(api::Error::UnsupportedProjectVersion)
        }
    }
}

fn update_version(db: &sqlite::Connection, version: &str) -> api::Result<()> {
    let mut stmt = db.prepare(UPDATE_VERSION)?;
    stmt.bind(1, version)?;
    if stmt.next()? != sqlite::State::Done {
        Err(api::Error::SQLError)
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    const fn new(major: u8, minor: u8, patch: u8) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }
}

impl FromStr for Version {
    type Err = api::Error;
    fn from_str(version: &str) -> Result<Self, Self::Err> {
        let version_parts: Vec<_> = version
            .splitn(3, '.')
            .map(|x| x.parse().map_err(|_| api::Error::UnsupportedProjectVersion))
            .collect();
        if version_parts.len() == 3 {
            Ok(Version {
                major: version_parts[0]?,
                minor: version_parts[1]?,
                patch: version_parts[2]?,
            })
        } else if version_parts.len() == 2 {
            Ok(Version {
                major: 0,
                minor: version_parts[0]?,
                patch: version_parts[1]?,
            })
        } else {
            Err(api::Error::UnsupportedProjectVersion)
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

fn patch_0_8_0(db: &sqlite::Connection) -> api::Result<()> {
    const UPDATE_MAIL_PLACEHOLDERS: &str = r#"
update sbv_meta set
value=replace(replace(value, '[mediumtitel]', '{booktitle}'), '[name]', '{username}')
where key like 'mail.%.subject' or key like 'mail.%.content'
"#;
    db.execute(UPDATE_MAIL_PLACEHOLDERS)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn version_parsing() {
        assert!("0.0".parse::<Version>().unwrap() == Version::new(0, 0, 0));
        assert!("1.0".parse::<Version>().unwrap() == Version::new(0, 1, 0));
        assert!("3.55".parse::<Version>().unwrap() == Version::new(0, 3, 55));
        assert!("0.0.0".parse::<Version>().unwrap() == Version::new(0, 0, 0));
        assert!("0.1.0".parse::<Version>().unwrap() == Version::new(0, 1, 0));
        assert!("0.9.22".parse::<Version>().unwrap() == Version::new(0, 9, 22));
        assert!("10.9.22".parse::<Version>().unwrap() == Version::new(10, 9, 22));
        assert!("255.255.255".parse::<Version>().unwrap() == Version::new(255, 255, 255));

        assert!("10".parse::<Version>().is_err());
        assert!("1.2.3.4".parse::<Version>().is_err());
        assert!("0.-1".parse::<Version>().is_err());
        assert!("1.2.-2".parse::<Version>().is_err());
        assert!("..".parse::<Version>().is_err());

        assert!(PKG_VERSION.parse::<Version>().is_ok());
    }

    #[test]
    fn create_tables() {
        let db = Database::memory().unwrap();
        db.structure_create(PKG_VERSION).unwrap();

        let books: Vec<Book> = db.book_search("").unwrap().collect();
        assert!(books.is_empty());
        let users: Vec<User> = db.user_search("").unwrap().collect();
        assert!(users.is_empty());
        let categories: Vec<Category> = db.category_list().unwrap().collect();
        assert!(categories.is_empty());
        let settings: Settings = db.settings_fetch().unwrap();
        assert!(settings == Settings::default());
    }

    #[test]
    fn migrate_0_8_0() {
        let db = Database::memory().unwrap();
        db.structure_create("7.0").unwrap();
        let settings = Settings {
            mail_info_subject: "[mediumtitel]' is back in the library".into(),
            mail_info_content: "Hallo [name],\nYou have shown interest in the book '[mediumtitel].".into(),
            mail_overdue_subject: "'[mediumtitel]' has expired".into(),
            mail_overdue_content: "Hello [name],\nThe borrowing period for the book '[mediumtitel]' has expired.".into(),
            mail_overdue2_subject: "Overdue fines for '[mediumtitel]'!".into(),
            mail_overdue2_content: "Hello [name],\nThe borrowing period for the book '[mediumtitel]' has expired two weeks ago.".into(),
            mail_host: "[mediumtitel] [name]".into(),
            ..Settings::default()
        };
        db.settings_update(&settings).unwrap();

        patch_0_8_0(&db.db).unwrap();

        let settings = db.settings_fetch().unwrap();
        assert!(settings.mail_info_subject == "{booktitle}' is back in the library");
        assert!(
            settings.mail_info_content
                == "Hallo {username},\nYou have shown interest in the book '{booktitle}."
        );
        assert!(settings.mail_overdue_subject == "'{booktitle}' has expired");
        assert!(
            settings.mail_overdue_content
                == "Hello {username},\nThe borrowing period for the book '{booktitle}' has expired."
        );
        assert!(settings.mail_overdue2_subject == "Overdue fines for '{booktitle}'!");
        assert!(
            settings.mail_overdue2_content
                == "Hello {username},\nThe borrowing period for the book '{booktitle}' has expired two weeks ago.");
        assert!(settings.mail_host == "[mediumtitel] [name]");
    }
}
