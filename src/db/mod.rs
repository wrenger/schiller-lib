use std::borrow::Cow;
use std::path::Path;
use std::ptr::addr_of;

use crate::error::{Error, Result};

pub mod book;
pub use book::{Book, BookAdvancedSearch, BookSearch, BookState};
pub mod category;
pub use category::Category;
pub mod lending;
pub mod settings;
pub use settings::Settings;
pub mod stats;
pub use stats::Stats;
pub mod structure;
pub mod user;
pub use user::{User, UserAdvancedSearch, UserSearch};

use super::PKG_VERSION;

#[derive(Debug)]
pub struct Database {
    con: rusqlite::Connection,
}

impl Database {
    /// Creates a new database at the given path.
    pub fn create(path: Cow<'_, Path>) -> Result<Database> {
        if !path.exists() {
            let database = Database {
                con: rusqlite::Connection::open_with_flags(
                    &path,
                    rusqlite::OpenFlags::SQLITE_OPEN_CREATE
                        | rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
                )
                .map_err(|_| Error::FileOpen)?,
            };
            structure::create(&database, PKG_VERSION)?;
            Ok(database)
        } else {
            Err(Error::FileOpen)
        }
    }

    /// Opens a database connection to the given project database.
    pub fn open(path: Cow<'_, Path>) -> Result<(Database, bool)> {
        if path.exists() {
            let database = Database {
                con: rusqlite::Connection::open_with_flags(
                    &path,
                    rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
                )
                .map_err(|_| Error::FileOpen)?,
            };
            let updated = structure::migrate(&database, PKG_VERSION)?;
            Ok((database, updated))
        } else {
            Err(Error::FileNotFound)
        }
    }

    /// In memory database for testing purposes.
    #[cfg(test)]
    fn memory() -> Result<Database> {
        Ok(Database {
            con: rusqlite::Connection::open_in_memory()?,
        })
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
