use std::fmt::{self, Display};
use std::str::FromStr;

use crate::api;

use super::{settings, settings::Settings, Database};

/// Minimum supported version.
const MIN_VERSION: Version = Version(0, 7, 0);

type Migration = fn(&Database) -> api::Result<()>;

/// Database migration routines
const PATCHES: [(Version, Migration); 2] = [
    (Version(0, 8, 0), patch_0_8_0),
    (Version(0, 8, 3), patch_0_8_3),
];

pub fn create(db: &Database, version: &str) -> api::Result<()> {
    const CREATE_TABLES: &str = "\
        create table sbv_meta ( \
        key text primary key, \
        value text not null); \
        \
        create table author ( \
        name text not null, \
        medium text not null, \
        primary key (name, medium)); \
        \
        create table user ( \
        account text not null primary key, \
        forename text not null, \
        surname text not null, \
        role text not null, \
        may_borrow integer not null default 1); \
        \
        create table category ( \
        id text not null primary key, \
        name text not null, \
        section text not null); \
        \
        create table medium ( \
        id text not null primary key, \
        isbn text not null, \
        title text not null, \
        publisher text not null, \
        year integer, \
        costs real, \
        note text not null, \
        borrowable integer not null, \
        category text not null, \
        borrower text not null default '', \
        deadline text not null default '', \
        reservation text not null default ''); \
    ";

    let transaction = db.transaction()?;
    transaction.execute_batch(CREATE_TABLES)?;
    update_version(&transaction, version)?;
    settings::update(db, &Settings::default())?;
    transaction.commit()?;
    Ok(())
}

/// Applies the related migration routines if the version changed.
/// Returns true if the database was updated.
pub fn migrate(db: &Database, version: &str) -> api::Result<bool> {
    let transaction = db.transaction()?;
    let old_version: String = transaction
        .query_row(
            "select value from sbv_meta where key='version'",
            [],
            |row| row.get(0),
        )
        .map_err(|_| api::Error::UnsupportedProjectVersion)?;
    info!("Start migration of {old_version}");

    let old_version: Version = old_version.parse()?;
    let new_version: Version = version.parse()?;
    if MIN_VERSION <= old_version && old_version <= new_version {
        for (patch_version, patch) in &PATCHES {
            if old_version < *patch_version {
                info!("Applying patch {patch_version}");
                patch(db)?;
            }
        }
        update_version(&transaction, version)?;
        transaction.commit()?;
        Ok(old_version != new_version)
    } else {
        Err(api::Error::UnsupportedProjectVersion)
    }
}

fn update_version(db: &rusqlite::Connection, version: &str) -> api::Result<()> {
    db.execute("replace into sbv_meta values ('version', ?)", [version])?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Version(u8, u8, u8);

impl FromStr for Version {
    type Err = api::Error;
    fn from_str(version: &str) -> Result<Self, Self::Err> {
        let version_parts = version
            .splitn(3, '.')
            .map(str::parse)
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_| api::Error::UnsupportedProjectVersion)?;
        if let [major, minor, patch] = version_parts[..] {
            Ok(Version(major, minor, patch))
        } else if let [minor, patch] = version_parts[..] {
            Ok(Version(0, minor, patch))
        } else {
            Err(api::Error::UnsupportedProjectVersion)
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

fn patch_0_8_0(db: &Database) -> api::Result<()> {
    const UPDATE_MAIL_PLACEHOLDERS: &str = "\
        update sbv_meta set \
        value=replace(replace(value, '[mediumtitel]', '{booktitle}'), '[name]', '{username}') \
        where key like 'mail.%.subject' or key like 'mail.%.content' \
    ";
    db.con.execute(UPDATE_MAIL_PLACEHOLDERS, [])?;
    Ok(())
}

fn patch_0_8_3(db: &Database) -> api::Result<()> {
    const UPDATE_USER_ROLES: &str = "\
        update user set \
        role=? \
        where role='' \
    ";
    db.con.execute(UPDATE_USER_ROLES, ["-"])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::PKG_VERSION;

    use super::super::*;
    use super::*;

    #[test]
    fn version_parsing() {
        assert!("0.0".parse::<Version>().unwrap() == Version(0, 0, 0));
        assert!("1.0".parse::<Version>().unwrap() == Version(0, 1, 0));
        assert!("3.55".parse::<Version>().unwrap() == Version(0, 3, 55));
        assert!("0.0.0".parse::<Version>().unwrap() == Version(0, 0, 0));
        assert!("0.1.0".parse::<Version>().unwrap() == Version(0, 1, 0));
        assert!("0.9.22".parse::<Version>().unwrap() == Version(0, 9, 22));
        assert!("10.9.22".parse::<Version>().unwrap() == Version(10, 9, 22));
        assert!("255.255.255".parse::<Version>().unwrap() == Version(255, 255, 255));

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
        structure::create(&db, PKG_VERSION).unwrap();

        let books = book::search(&db, "").unwrap();
        assert!(books.is_empty());
        let users = user::search(&db, "").unwrap();
        assert!(users.is_empty());
        let categories = category::list(&db).unwrap();
        assert!(categories.is_empty());
        let settings: Settings = settings::fetch(&db).unwrap();
        assert!(settings == Settings::default());
    }

    #[test]
    fn migrate_0_8_0() {
        let db = Database::memory().unwrap();
        structure::create(&db, "7.0").unwrap();
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
        settings::update(&db, &settings).unwrap();

        patch_0_8_0(&db).unwrap();

        let settings = settings::fetch(&db).unwrap();
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

    #[test]
    fn migrate_0_8_3() {
        let db = Database::memory().unwrap();
        structure::create(&db, "0.8.2").unwrap();

        let u1 = User {
            account: "max.mueller".into(),
            forename: "Max".into(),
            surname: "Mueller".into(),
            role: "".into(),
            may_borrow: true,
        };
        let u2 = User {
            account: "john.doe".into(),
            forename: "John".into(),
            surname: "Doe".into(),
            role: "VIP".into(),
            may_borrow: true,
        };

        // Circumvent valid check
        db.con.execute(
        "insert into user values (?, ?, ?, ?, ?)",
            rusqlite::params![
                u1.account.trim(),
                u1.forename.trim(),
                u1.surname.trim(),
                u1.role.trim(),
                u1.may_borrow as i64,
            ],
        ).unwrap();

        user::add(&db, &u2).unwrap();

        patch_0_8_3(&db).unwrap();

        let mut new_u1 = user::fetch(&db, "max.mueller").unwrap();
        assert_eq!(&new_u1.role, "-");
        new_u1.role.clear();
        assert_eq!(u1, new_u1);

        let new_u2 = user::fetch(&db, "john.doe").unwrap();
        assert_eq!(u2, new_u2);
    }
}
