use std::cmp::Ordering;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{fs::File, io::BufWriter};

use chrono::{Local, NaiveDate};
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
#[deprecated]
mod legacy;
mod sorted;

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
    pub mail_info_subject: String,
    pub mail_info_content: String,
    pub mail_overdue_subject: String,
    pub mail_overdue_content: String,
    pub mail_overdue2_subject: String,
    pub mail_overdue2_content: String,
}

impl Settings {
    fn validate(&mut self) -> bool {
        self.dnb_token = self.dnb_token.trim().to_string();
        self.mail_from = self.mail_from.trim().to_string();
        self.mail_host = self.mail_host.trim().to_string();
        self.mail_password = self.mail_password.trim().to_string();
        self.mail_info_subject = self.mail_info_subject.trim().to_string();
        self.mail_info_content = self.mail_info_content.trim().to_string();
        self.mail_overdue_subject = self.mail_overdue_subject.trim().to_string();
        self.mail_overdue_content = self.mail_overdue_content.trim().to_string();
        self.mail_overdue2_subject = self.mail_overdue2_subject.trim().to_string();
        self.mail_overdue2_content = self.mail_overdue2_content.trim().to_string();
        self.mail_from.is_empty() || account_is_valid(&self.mail_from)
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            borrowing_duration: 28,
            dnb_token: String::new(),
            mail_last_reminder: Local::now().naive_local().date(),
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
    pub fn load(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }

    pub fn save(&self, file: &Path) -> Result<()> {
        let writer = BufWriter::new(File::create(file)?);
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
        for book in self.books.data.values() {
            if !book.borrower.is_empty() {
                borrows += 1;
            }
            if !book.reservation.is_empty() {
                reservations += 1;
            }

            let now = Local::now().naive_local().date();
            if let Some(deadline) = book.deadline {
                if now > deadline {
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
        if !book.borrower.is_empty() && book.borrower != user.account {
            return Err(Error::LendingBookAlreadyBorrowed);
        }
        if !book.reservation.is_empty() {
            if book.reservation == user.account {
                book = self.release(id)?; // Allow lending to reserver
            } else {
                return Err(Error::LendingBookAlreadyReserved);
            }
        }

        book.borrower = user.account.clone();
        book.deadline = Some(deadline);
        self.books.update(id, book, &mut self.categories)
    }

    /// Returns the book.
    pub fn return_back(&mut self, id: &str) -> Result<Book> {
        let mut book = self.books.fetch(id)?;

        if book.borrower.is_empty() {
            return Err(Error::LendingBookNotBorrowed);
        }

        book.borrower = String::new();
        book.deadline = None;
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
        if !book.reservation.is_empty() {
            return Err(Error::LendingBookAlreadyReserved);
        }
        if book.borrower.is_empty() {
            return Err(Error::LendingBookNotBorrowed);
        }
        if book.borrower == user.account {
            return Err(Error::LendingBookAlreadyBorrowedByUser);
        }

        book.reservation = user.account.clone();
        self.books.update(id, book, &mut self.categories)
    }
    /// Removes the reservation from the specified book.
    pub fn release(&mut self, id: &str) -> Result<Book> {
        let mut book = self.books.fetch(id)?;

        if book.reservation.is_empty() {
            return Err(Error::LendingBookNotReserved);
        }

        book.reservation = String::new();
        self.books.update(id, book, &mut self.categories)
    }

    /// Return the list of expired loan periods.
    pub fn overdues(&self) -> Result<Vec<(Book, User)>> {
        fn sort(a: &(Book, User), b: &(Book, User)) -> Ordering {
            a.0.deadline
                .cmp(&b.0.deadline)
                .then_with(|| a.0.id.cmp(&b.0.id))
        }

        let mut results = Sorted::new(sort);

        let now = Local::now().naive_local().date();
        for book in self.books.data.values() {
            if let Some(deadline) = book.deadline {
                if now > deadline {
                    let user = self.users.fetch(&book.borrower)?;
                    results.push((book.clone(), user));
                }
            }
        }
        Ok(results.into_iter().collect())
    }
}

/// Synchronized Wrapper, that automatically saves changes
pub struct AtomicDatabase {
    file: PathBuf,
    data: RwLock<Database>,
}

impl AtomicDatabase {
    pub fn load(file: &Path) -> Result<Self> {
        let (file, data) = migrate::import(file)?;
        Ok(Self {
            file,
            data: RwLock::new(data),
        })
    }

    pub fn create(file: &Path) -> Result<Self> {
        let data = Database::default();
        data.save(file)?;
        Ok(Self {
            file: file.into(),
            data: RwLock::new(data),
        })
    }

    pub fn read(&self) -> RwLockReadGuard<'_, Database> {
        self.data.read().unwrap()
    }
    pub fn write(&self) -> AtomicDatabaseWrite<'_> {
        AtomicDatabaseWrite(self.data.write().unwrap(), &self.file)
    }
}
pub struct AtomicDatabaseWrite<'a>(RwLockWriteGuard<'a, Database>, &'a Path);
impl Deref for AtomicDatabaseWrite<'_> {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AtomicDatabaseWrite<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Drop for AtomicDatabaseWrite<'_> {
    fn drop(&mut self) {
        info!("Saving database");
        self.0.save(self.1).unwrap()
    }
}
impl fmt::Debug for AtomicDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AtomicDatabase")
            .field("file", &self.file)
            .finish()
    }
}

#[cfg(test)]
mod test {
    #[allow(deprecated)]
    #[test]
    fn compare_times() {
        use std::hint::black_box;
        use std::path::Path;
        use std::time::Instant;

        use super::legacy as d1;
        use crate::db as d2;
        use tracing::info;

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

        db2.save(&file.with_extension("json")).unwrap();

        info!("db2: {:?}", db2.stats());
    }
}
