use std::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::{self, BufWriter, Seek};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use chrono::{Local, NaiveDate};
use fs4::FileExt;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::db::sorted::Sorted;
use crate::error::{Error, Result};
use crate::mail::account_is_valid;

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

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Settings {
    // Borrowing
    pub borrowing_duration: i64,
    // DNB
    pub dnb_token: String,
    // Mail
    pub mail_last_reminder: NaiveDate,
    pub mail_from: String,
    pub mail_host: String,
    pub mail_password: String,
    // Mail Templates
    pub mail_info: MailTemplate,
    pub mail_overdue: MailTemplate,
    pub mail_overdue2: MailTemplate,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct MailTemplate {
    pub subject: String,
    pub body: String,
}

impl Settings {
    fn validate(&mut self) -> bool {
        self.dnb_token = self.dnb_token.trim().to_string();
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
            dnb_token: Default::default(),
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

#[derive(Serialize, Deserialize)]
pub struct Database {
    version: Version,
    pub books: Books,
    pub users: Users,
    pub categories: Categories,
    settings: Settings,
}

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
            version: crate::PKG_VERSION.parse().unwrap(),
            books: Default::default(),
            users: Default::default(),
            categories: Default::default(),
            settings: Default::default(),
        }
    }
}

impl Database {
    pub fn load(file: impl io::Read) -> Result<Self> {
        Ok(serde_json::from_reader(file)?)
    }

    pub fn save(&self, file: impl io::Write) -> Result<()> {
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    pub fn settings(&self) -> Settings {
        self.settings.clone()
    }

    pub fn settings_update(&mut self, mut settings: Settings) -> Result<Settings> {
        if settings.validate() {
            self.settings = settings.clone();
            Ok(settings)
        } else {
            Err(Error::Arguments)
        }
    }

    pub fn stats(&self) -> Result<Stats> {
        let mut borrows = 0;
        let mut reservations = 0;
        let mut overdues = 0;

        let now = Local::now().naive_local().date();

        for book in self.books.data.values() {
            if !book.borrower.is_none() {
                borrows += 1;
            }
            if !book.reservation.is_none() {
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
            deadline: deadline,
        });
        self.books.update(id, book, &mut self.categories)
    }

    /// Returns the book.
    pub fn return_back(&mut self, id: &str) -> Result<Book> {
        let mut book = self.books.fetch(id)?;

        if book.borrower.is_none() {
            return Err(Error::LendingBookNotBorrowed);
        }

        book.borrower = None;
        self.books.update(id, book, &mut self.categories)
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
        self.books.update(id, book, &mut self.categories)
    }
    /// Removes the reservation from the specified book.
    pub fn release(&mut self, id: &str) -> Result<Book> {
        let mut book = self.books.fetch(id)?;

        if book.reservation.is_none() {
            return Err(Error::LendingBookNotReserved);
        }

        book.reservation = None;
        self.books.update(id, book, &mut self.categories)
    }

    /// Return the list of expired loan periods.
    pub fn overdues(&self) -> Result<Vec<Overdue>> {
        let mut results = Sorted::new(|a: &Overdue, b| a.cmp(b));

        let now = Local::now().naive_local().date();
        for book in self.books.data.values() {
            if let Some(borrower) = &book.borrower {
                if now > borrower.deadline {
                    let user = self.users.fetch(&borrower.user)?;
                    results.push(Overdue {
                        book: book.clone(),
                        user,
                    });
                }
            }
        }
        Ok(results.into_iter().collect())
    }
}

/// Synchronized Wrapper, that automatically saves changes
///
/// It also locks the database file, preventing other applications from accessing it.
pub struct AtomicDatabase {
    path: PathBuf,
    data: RwLock<(File, Database)>,
}

impl AtomicDatabase {
    pub fn load(path: &Path) -> Result<Self> {
        let (file, data) = migrate::import(path)?;
        Ok(Self {
            path: path.into(),
            data: RwLock::new((file, data)),
        })
    }

    pub fn create(path: &Path) -> Result<Self> {
        let data = Database::default();
        let mut file = File::create(path)?;
        file.try_lock_exclusive()?;
        data.save(&mut file)?;
        Ok(Self {
            path: path.into(),
            data: RwLock::new((file, data)),
        })
    }

    pub fn read(&self) -> AtomicDatabaseRead<'_> {
        AtomicDatabaseRead(self.data.read().unwrap())
    }
    pub fn write(&self) -> AtomicDatabaseWrite<'_> {
        AtomicDatabaseWrite(self.data.write().unwrap())
    }
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
        let mut guard = self.data.write().unwrap();
        let (file, data) = &mut *guard;
        // truncate
        file.rewind().unwrap();
        file.set_len(0).unwrap();
        data.save(file).unwrap();
        // unlock file
        guard.0.unlock().unwrap();
    }
}

pub struct AtomicDatabaseRead<'a>(RwLockReadGuard<'a, (File, Database)>);
impl Deref for AtomicDatabaseRead<'_> {
    type Target = Database;
    fn deref(&self) -> &Self::Target {
        &self.0 .1
    }
}

pub struct AtomicDatabaseWrite<'a>(RwLockWriteGuard<'a, (File, Database)>);
impl Deref for AtomicDatabaseWrite<'_> {
    type Target = Database;
    fn deref(&self) -> &Self::Target {
        &self.0 .1
    }
}
impl DerefMut for AtomicDatabaseWrite<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0 .1
    }
}
impl Drop for AtomicDatabaseWrite<'_> {
    fn drop(&mut self) {
        info!("Saving database");
        let (file, data) = &mut *self.0;
        // truncate
        file.rewind().unwrap();
        file.set_len(0).unwrap();
        data.save(file).unwrap();
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

        crate::logging();
        let file = Path::new("test/data/schillerbib.db");

        let db1 = d1::Database::open(file.into()).unwrap().0;
        let db2 = super::migrate::import(file).unwrap().1;

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
