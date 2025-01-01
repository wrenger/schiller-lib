use std::cmp::Ordering;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, BufWriter};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{fmt, fs};

use chrono::{Local, NaiveDate};
use gluer::metadata;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::db::sorted::Sorted;
use crate::error::{Error, Result};
use crate::mail::account_is_valid;
use crate::util::PKG_VERSION;

mod book;
pub use book::*;
mod user;
pub use user::*;
mod category;
pub use category::*;
mod migrate;
pub use migrate::Version;
mod sorted;

#[cfg(feature = "sqlite")]
#[deprecated]
mod legacy;

/// Library settings
#[metadata]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Settings {
    // Borrowing
    pub borrowing_duration: i64,
    // Mail
    #[meta(into = String)]
    pub mail_last_reminder: NaiveDate,
    pub mail_from: String,
    pub mail_host: String,
    // TODO: Redact from requests
    pub mail_password: String,
    // Mail Templates
    pub mail_info: MailTemplate,
    pub mail_overdue: MailTemplate,
    pub mail_overdue2: MailTemplate,
}

/// Template for a mail notification
#[metadata]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct MailTemplate {
    pub subject: String,
    pub body: String,
}

impl Settings {
    fn validate(&mut self) -> bool {
        self.mail_from = self.mail_from.trim().to_string();
        self.mail_host = self.mail_host.trim().to_string();
        self.mail_password = self.mail_password.trim().to_string();
        self.mail_info.subject = self.mail_info.subject.trim().to_string();
        self.mail_info.body = self.mail_info.body.trim().to_string();
        self.mail_overdue.subject = self.mail_overdue.subject.trim().to_string();
        self.mail_overdue.body = self.mail_overdue.body.trim().to_string();
        self.mail_overdue2.subject = self.mail_overdue2.subject.trim().to_string();
        self.mail_overdue2.body = self.mail_overdue2.body.trim().to_string();
        self.mail_from.is_empty() || account_is_valid(&self.mail_from)
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            borrowing_duration: 28,
            mail_last_reminder: Local::now().naive_local().date(),
            mail_from: Default::default(),
            mail_host: Default::default(),
            mail_password: Default::default(),
            mail_info: Default::default(),
            mail_overdue: Default::default(),
            mail_overdue2: Default::default(),
        }
    }
}

/// Data object for book.
#[metadata]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct Stats {
    pub books: usize,
    pub users: usize,
    pub categories: usize,
    pub borrows: usize,
    pub reservations: usize,
    pub overdues: usize,
}

/// Library database
#[derive(Serialize, Deserialize)]
pub struct Database {
    version: Version,
    pub books: Books,
    pub users: Users,
    pub categories: Categories,
    settings: Settings,
}

/// Borrowed books that missed the deadline
#[metadata]
#[derive(Serialize)]
pub struct Overdue {
    pub book: Book,
    pub user: User,
}
impl PartialEq for Overdue {
    fn eq(&self, other: &Self) -> bool {
        self.book.id == other.book.id
    }
}
impl Eq for Overdue {}
impl PartialOrd for Overdue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Overdue {
    fn cmp(&self, other: &Self) -> Ordering {
        if let (Some(borrower), Some(other_borrower)) = (&self.book.borrower, &other.book.borrower)
        {
            match borrower.deadline.cmp(&other_borrower.deadline) {
                o @ (Ordering::Greater | Ordering::Less) => return o,
                Ordering::Equal => {}
            }
        }
        self.book.id.cmp(&other.book.id)
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            version: PKG_VERSION.parse().unwrap(),
            books: Default::default(),
            users: Default::default(),
            categories: Default::default(),
            settings: Default::default(),
        }
    }
}

impl Database {
    /// Load a database from a file
    pub fn load(file: impl io::Read) -> Result<Self> {
        Ok(serde_json::from_reader(file)?)
    }
    /// Save this database to a file
    pub fn save(&self, file: impl io::Write) -> Result<()> {
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }
    /// Return the library settings
    pub fn settings(&self) -> Settings {
        self.settings.clone()
    }
    /// Save the given settings to the database
    pub fn settings_update(&mut self, mut settings: Settings) -> Result<Settings> {
        if settings.validate() {
            self.settings = settings.clone();
            Ok(settings)
        } else {
            Err(Error::Arguments)
        }
    }

    /// Return library statistics
    pub fn stats(&self) -> Result<Stats> {
        let mut borrows = 0;
        let mut reservations = 0;
        let mut overdues = 0;

        let now = Local::now().naive_local().date();

        for book in self.books.data.values() {
            if book.borrower.is_some() {
                borrows += 1;
            }
            if book.reservation.is_some() {
                reservations += 1;
            }

            if let Some(borrower) = &book.borrower {
                if now > borrower.deadline {
                    overdues += 1;
                }
            }
        }

        Ok(Stats {
            books: self.books.data.len(),
            users: self.users.data.len(),
            categories: self.categories.data.len(),
            borrows,
            reservations,
            overdues,
        })
    }

    /// Lend the given book (`id`) to the `account`
    pub fn lend(&mut self, id: &str, account: &str, deadline: NaiveDate) -> Result<Book> {
        let mut book = self.books.fetch(id)?;
        let user = self.users.fetch(account)?;

        if !user.may_borrow {
            return Err(Error::LendingUserMayNotBorrow);
        }
        if !book.borrowable {
            return Err(Error::LendingBookNotBorrowable);
        }
        // Allow renewal
        if book.borrower.is_some_and(|b| b.user != user.account) {
            return Err(Error::LendingBookAlreadyBorrowed);
        }
        if let Some(reservation) = &book.reservation {
            if *reservation == user.account {
                book = self.release(id)?; // Allow lending to reserver
            } else {
                return Err(Error::LendingBookAlreadyReserved);
            }
        }

        book.borrower = Some(Borrower {
            user: user.account.clone(),
            deadline,
        });
        self.books.update(id, book, &self.categories)
    }
    /// Returns the book.
    pub fn return_back(&mut self, id: &str) -> Result<Book> {
        let mut book = self.books.fetch(id)?;

        if book.borrower.is_none() {
            return Err(Error::LendingBookNotBorrowed);
        }

        book.borrower = None;
        self.books.update(id, book, &self.categories)
    }
    /// Creates a reservation for the borrowed book.
    pub fn reserve(&mut self, id: &str, account: &str) -> Result<Book> {
        let mut book = self.books.fetch(id)?;
        let user = self.users.fetch(account)?;

        if !user.may_borrow {
            return Err(Error::LendingUserMayNotBorrow);
        }
        if !book.borrowable {
            return Err(Error::LendingBookNotBorrowable);
        }
        if book.reservation.is_some() {
            return Err(Error::LendingBookAlreadyReserved);
        }
        if book.borrower.is_none() {
            return Err(Error::LendingBookNotBorrowed);
        }
        if book
            .borrower
            .as_ref()
            .is_some_and(|b| b.user == user.account)
        {
            return Err(Error::LendingBookAlreadyBorrowedByUser);
        }

        book.reservation = Some(user.account.clone());
        self.books.update(id, book, &self.categories)
    }
    /// Removes the reservation from the specified book.
    pub fn release(&mut self, id: &str) -> Result<Book> {
        let mut book = self.books.fetch(id)?;

        if book.reservation.is_none() {
            return Err(Error::LendingBookNotReserved);
        }

        book.reservation = None;
        self.books.update(id, book, &self.categories)
    }

    /// Return the list of expired loan periods.
    pub fn overdues(&self) -> Result<Vec<Overdue>> {
        let mut results = Sorted::new(Overdue::cmp);

        let now = Local::now().naive_local().date();
        for book in self.books.data.values() {
            if let Some(borrower) = &book.borrower {
                if now > borrower.deadline {
                    results.push(Overdue {
                        book: book.clone(),
                        user: self.users.fetch(&borrower.user)?,
                    });
                }
            }
        }
        Ok(results.into_iter().collect())
    }
}

/// Synchronized Wrapper, that automatically saves changes
pub struct AtomicDatabase {
    path: PathBuf,
    /// Name of the DB temporary file
    tmp: PathBuf,
    data: RwLock<Database>,
}

impl AtomicDatabase {
    /// Load the database from the file system.
    ///
    /// This also migrates it if it necessary.
    pub fn load(path: &Path) -> Result<Self> {
        let new_path = path.with_extension("json");
        let tmp = Self::tmp_path(&new_path)?;

        let data = migrate::import(path)?;
        atomic_write(&tmp, &new_path, &data)?;

        Ok(Self {
            path: new_path,
            tmp,
            data: RwLock::new(data),
        })
    }
    /// Create a new database and save it.
    pub fn create(path: &Path) -> Result<Self> {
        let tmp = Self::tmp_path(path)?;

        let data = Database::default();
        atomic_write(&tmp, path, &data)?;

        Ok(Self {
            path: path.into(),
            tmp,
            data: RwLock::new(data),
        })
    }
    /// Lock the database for reading.
    pub fn read(&self) -> AtomicDatabaseRead<'_> {
        AtomicDatabaseRead {
            data: self.data.read().unwrap(),
        }
    }
    /// Lock the database for writing. This will save the changes atomically on drop.
    pub fn write(&self) -> AtomicDatabaseWrite<'_> {
        AtomicDatabaseWrite {
            path: &self.path,
            tmp: &self.tmp,
            data: self.data.write().unwrap(),
        }
    }

    fn tmp_path(path: &Path) -> Result<PathBuf> {
        let mut tmp_name = OsString::from(".");
        tmp_name.push(path.file_name().unwrap_or(OsStr::new("db")));
        tmp_name.push("~");
        let tmp = path.with_file_name(tmp_name);
        if tmp.exists() {
            error!(
                "Found orphaned database temporary file '{tmp:?}'. The server has recently crashed or is already running. Delete this before continuing!"
            );
            return Err(Error::FileOpen);
        }
        Ok(tmp)
    }
}

/// Atomic write routine, loosely inspired by the tempfile crate.
///
/// This assumes that the rename FS operations are atomic.
fn atomic_write(tmp: &Path, path: &Path, data: &Database) -> Result<()> {
    {
        let mut tmpfile = File::create_new(tmp)?;
        data.save(&mut tmpfile)?;
        tmpfile.sync_all()?; // just to be sure!
    }
    fs::rename(tmp, path)?;
    Ok(())
}

impl fmt::Debug for AtomicDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AtomicDatabase")
            .field("file", &self.path)
            .finish()
    }
}

impl Drop for AtomicDatabase {
    fn drop(&mut self) {
        info!("Saving database");
        let guard = self.data.read().unwrap();
        atomic_write(&self.tmp, &self.path, &guard).unwrap();
    }
}

pub struct AtomicDatabaseRead<'a> {
    data: RwLockReadGuard<'a, Database>,
}
impl Deref for AtomicDatabaseRead<'_> {
    type Target = Database;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub struct AtomicDatabaseWrite<'a> {
    tmp: &'a Path,
    path: &'a Path,
    data: RwLockWriteGuard<'a, Database>,
}
impl Deref for AtomicDatabaseWrite<'_> {
    type Target = Database;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DerefMut for AtomicDatabaseWrite<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl Drop for AtomicDatabaseWrite<'_> {
    fn drop(&mut self) {
        info!("Saving database");
        atomic_write(self.tmp, self.path, &self.data).unwrap();
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "sqlite")]
    #[allow(deprecated)]
    #[test]
    #[ignore]
    fn compare_times() {
        use std::fs::File;
        use std::hint::black_box;
        use std::path::Path;
        use std::time::Instant;

        use tracing::info;

        use super::legacy as d1;
        use crate::db as d2;

        crate::util::logging();
        let file = Path::new("test/schillerbib.db");

        let db1 = d1::Database::open(file.into()).unwrap().0;
        let db2 = super::migrate::import(file).unwrap();

        let timer = Instant::now();
        let results = db1.books().unwrap();
        info!(
            "all d1: {}us for {:?}",
            timer.elapsed().as_micros(),
            results.len()
        );
        black_box(results);

        let timer = Instant::now();
        let results = db1.books().unwrap();
        info!(
            "all d1: {}us for {:?}",
            timer.elapsed().as_micros(),
            results.len()
        );
        black_box(results);

        let params = d2::BookSearch {
            query: String::new(),
            category: String::new(),
            state: d2::BookState::None,
            offset: 0,
            limit: usize::MAX,
        };

        let timer = Instant::now();
        let results = db2.books.search(black_box(&params)).unwrap();
        info!(
            "all d2: {}us for {}",
            timer.elapsed().as_micros(),
            results.0
        );
        assert_eq!(results.0, results.1.len());
        black_box(results);

        let timer = Instant::now();
        let results = db2.books.search(black_box(&params)).unwrap();
        info!(
            "all d2: {}us for {}",
            timer.elapsed().as_micros(),
            results.0
        );
        assert_eq!(results.0, results.1.len());
        black_box(results);

        db2.save(File::create(file.with_extension("json")).unwrap())
            .unwrap();

        info!("db2: {:?}", db2.stats());
    }
}
