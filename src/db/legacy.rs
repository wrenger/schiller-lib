use std::path::Path;
use std::ptr::addr_of;

use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::db::Version;
use crate::error::{Error, Result};
use crate::util::PKG_VERSION;

#[derive(Debug)]
pub struct Database {
    con: rusqlite::Connection,
}

impl Database {
    /// Opens a database connection to the given project database.
    pub fn open(path: &Path) -> Result<(Database, bool)> {
        if path.exists() {
            let database = Database {
                con: rusqlite::Connection::open_with_flags(
                    path,
                    rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
                )
                .map_err(|_| Error::FileOpen)?,
            };
            let updated = migrate(&database, PKG_VERSION)?;
            Ok((database, updated))
        } else {
            Err(Error::FileOpen)
        }
    }

    /// Creates a rollback point.
    /// If any statement on a transaction fails, all changes are rolled back
    /// to the point before this function is called.
    ///
    /// ## Safety
    /// This operation is only safe if called once.
    /// Stacking transactions on top of each other is not allowed!
    fn transaction(&self) -> rusqlite::Result<rusqlite::Transaction> {
        #[allow(invalid_reference_casting)]
        let con = unsafe { &mut *(addr_of!(self.con).cast_mut()) };
        con.transaction()
    }

    pub fn settings(&self) -> Result<Settings> {
        let mut stmt = self.con.prepare("select key, value from sbv_meta")?;
        let rows = stmt.query([])?;
        DBIter::new(rows).collect()
    }

    /// Performs a simple media search with the given `text`.
    pub fn books(&self) -> Result<Vec<Book>> {
        let mut stmt = self.con.prepare(
            "SELECT \
            id, \
            isbn, \
            title, \
            publisher, \
            year, \
            costs, \
            note, \
            borrowable, \
            category, \
            ifnull(group_concat(author.name), '') as authors, \
            borrower, \
            deadline, \
            reservation, \
            count(*) over() as total_count \
            \
            from medium \
            left join author on author.medium=id \
            group by id",
        )?;

        let rows = stmt.query([])?;
        DBIter::new(rows).collect()
    }

    /// Performes a simple user search with the given `text`.
    pub fn users(&self) -> Result<Vec<User>> {
        let mut stmt = self.con.prepare(
            "select \
            account, \
            forename, \
            surname, \
            role, \
            may_borrow, \
            count(*) over() as total_count \
            \
            from user",
        )?;
        let rows = stmt.query([])?;
        DBIter::new(rows).collect()
    }

    /// Returns all categories.
    pub fn categories(&self) -> Result<Vec<Category>> {
        let mut stmt = self
            .con
            .prepare("select id, name, section from category order by section, id")?;
        let rows = stmt.query([])?;
        DBIter::new(rows).collect()
    }
}

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

impl From<Settings> for super::Settings {
    fn from(value: Settings) -> Self {
        Self {
            borrowing_duration: value.borrowing_duration as _,
            mail_last_reminder: NaiveDate::parse_from_str(&value.mail_last_reminder, "%Y-%m-%d")
                .unwrap_or_else(|_| Local::now().naive_local().date()),
            mail_from: value.mail_from,
            mail_host: value.mail_host,
            mail_password: value.mail_password,
            mail_info: super::MailTemplate {
                subject: value.mail_info_subject,
                body: value.mail_info_content,
            },
            mail_overdue: super::MailTemplate {
                subject: value.mail_overdue_subject,
                body: value.mail_overdue_content,
            },
            mail_overdue2: super::MailTemplate {
                subject: value.mail_overdue2_subject,
                body: value.mail_overdue2_content,
            },
            ..Default::default()
        }
    }
}

/// Data object for book.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct Book {
    pub id: String,
    pub isbn: String,
    pub title: String,
    pub publisher: String,
    pub year: i64,
    pub costs: f64,
    pub note: String,
    pub borrowable: bool,
    pub category: String,
    pub authors: Vec<String>,
    pub borrower: String,
    pub deadline: String,
    pub reservation: String,
}

impl FromRow for Book {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Book> {
        Ok(Book {
            id: row.get("id")?,
            isbn: row.get("isbn")?,
            title: row.get("title")?,
            publisher: row.get("publisher")?,
            year: row.get("year")?,
            costs: row.get("costs")?,
            note: row.get("note")?,
            borrowable: row.get("borrowable")?,
            category: row.get("category")?,
            authors: row
                .get::<&str, String>("authors")?
                .split(',')
                .map(ToString::to_string)
                .collect(),
            borrower: row.get("borrower")?,
            deadline: row.get("deadline")?,
            reservation: row.get("reservation")?,
        })
    }
}

impl From<Book> for super::Book {
    fn from(value: Book) -> Self {
        Self {
            id: value.id,
            isbn: value.isbn,
            title: value.title,
            publisher: value.publisher,
            year: value.year,
            costs: value.costs,
            note: value.note,
            borrowable: value.borrowable,
            category: value.category,
            authors: value.authors.join(", "),
            borrower: if value.deadline.is_empty() {
                None
            } else {
                Some(super::Borrower {
                    user: value.borrower,
                    deadline: value.deadline.parse().unwrap(),
                })
            },
            reservation: (!value.reservation.is_empty()).then_some(value.reservation),
        }
    }
}

/// Data object for a user.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct User {
    pub account: String,
    pub forename: String,
    pub surname: String,
    pub role: String,
    pub may_borrow: bool,
}

impl FromRow for User {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<User> {
        Ok(User {
            account: row.get("account")?,
            forename: row.get("forename")?,
            surname: row.get("surname")?,
            role: row.get("role")?,
            may_borrow: row.get("may_borrow")?,
        })
    }
}

impl From<User> for super::User {
    fn from(value: User) -> Self {
        Self {
            account: value.account,
            forename: value.forename,
            surname: value.surname,
            role: value.role,
            may_borrow: value.may_borrow,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub section: String,
}

impl FromRow for Category {
    fn from_row(rows: &rusqlite::Row) -> rusqlite::Result<Category> {
        Ok(Category {
            id: rows.get("id")?,
            name: rows.get("name")?,
            section: rows.get("section")?,
        })
    }
}

impl From<Category> for super::Category {
    fn from(value: Category) -> Self {
        Self {
            id: value.id,
            name: value.name,
            section: value.section,
        }
    }
}

/// Iterator over database results.
pub struct DBIter<'a, T> {
    rows: rusqlite::Rows<'a>,
    ty: std::marker::PhantomData<T>,
}

impl<'a, T> DBIter<'a, T> {
    pub fn new(rows: rusqlite::Rows<'a>) -> Self {
        DBIter {
            rows,
            ty: std::marker::PhantomData,
        }
    }
}

/// Conversion from database entries.
pub trait FromRow: Sized {
    fn from_row(stmt: &rusqlite::Row) -> rusqlite::Result<Self>;
}

impl<'a, T: FromRow> Iterator for DBIter<'a, T> {
    type Item = Result<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.rows.next() {
            Ok(row) => Some(T::from_row(row?).map_err(Into::into)),
            Err(e) => Some(Err(e.into())),
        }
    }
}

/// Minimum supported version.
const MIN_VERSION: Version = Version(0, 7, 0);

type Migration = fn(&Database) -> Result<()>;

/// Database migration routines
const PATCHES: [(Version, Migration); 3] = [
    (Version(0, 8, 0), patch_0_8_0),
    (Version(0, 8, 3), patch_0_8_3),
    (Version(0, 8, 4), patch_0_8_4),
];

/// Applies the related migration routines if the version changed.
/// Returns true if the database was updated.
fn migrate(db: &Database, version: &str) -> Result<bool> {
    let transaction = db.transaction()?;
    let old_version: String = transaction
        .query_row(
            "select value from sbv_meta where key='version'",
            [],
            |row| row.get(0),
        )
        .map_err(|_| Error::UnsupportedProjectVersion)?;
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
        Err(Error::UnsupportedProjectVersion)
    }
}

fn update_version(db: &rusqlite::Connection, version: &str) -> Result<()> {
    db.execute("replace into sbv_meta values ('version', ?)", [version])?;
    Ok(())
}

fn patch_0_8_0(db: &Database) -> Result<()> {
    const UPDATE_MAIL_PLACEHOLDERS: &str = "\
        update sbv_meta set \
        value=replace(replace(value, '[mediumtitel]', '{booktitle}'), '[name]', '{username}') \
        where key like 'mail.%.subject' or key like 'mail.%.content' \
    ";
    db.con.execute(UPDATE_MAIL_PLACEHOLDERS, [])?;
    Ok(())
}

fn patch_0_8_3(db: &Database) -> Result<()> {
    const UPDATE_USER_ROLES: &str = "\
        update user set \
        role=? \
        where role='' \
    ";
    db.con.execute(UPDATE_USER_ROLES, ["-"])?;
    Ok(())
}

fn patch_0_8_4(db: &Database) -> Result<()> {
    const DELETE_PATH: &str = "\
    delete from sbv_meta where key = ? \
    ";

    db.con.execute(DELETE_PATH, ["user.path"])?;
    const DELETE_DELIMITER: &str = "\
    delete from sbv_meta where key = ? \
    ";

    db.con.execute(DELETE_DELIMITER, ["user.delimiter"])?;
    Ok(())
}
