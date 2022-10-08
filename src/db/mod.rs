use std::fmt;
use std::path::{Path, PathBuf};
use std::ptr::addr_of;

use crate::api;

pub mod book;
pub use book::{Book, BookSearch, BookState};
pub mod category;
pub use category::Category;
pub mod lending;
pub mod settings;
pub use settings::Settings;
pub mod stats;
pub use stats::Stats;
pub mod structure;
pub mod user;
pub use user::User;

use super::PKG_VERSION;

use rusqlite::Connection;

pub struct Database {
    path: PathBuf,
    con: rusqlite::Connection,
}

impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Database")
            .field("path", &self.path)
            .finish()
    }
}

impl Database {
    /// Creates a new database at the given path.
    pub fn create(path: &str) -> api::Result<Database> {
        let path = PathBuf::from(path);
        if !path.exists() {
            let database = Database {
                con: rusqlite::Connection::open_with_flags(
                    &path,
                    rusqlite::OpenFlags::SQLITE_OPEN_CREATE
                        | rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
                )
                .map_err(|_| api::Error::FileOpen)?,
                path,
            };
            structure::create(&database, PKG_VERSION)?;
            Ok(database)
        } else {
            Err(api::Error::FileOpen)
        }
    }

    /// Opens a database connection to the given project database.
    pub fn open(path: &str) -> api::Result<(Database, bool)> {
        let path = PathBuf::from(path);
        if path.exists() {
            let database = Database {
                con: rusqlite::Connection::open_with_flags(
                    &path,
                    rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
                )
                .map_err(|_| api::Error::FileOpen)?,
                path,
            };
            let updated = structure::migrate(&database, PKG_VERSION)?;
            Ok((database, updated))
        } else {
            Err(api::Error::FileNotFound)
        }
    }

    /// Returns the filepath to this database.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// In memory database for testing purposes.
    #[cfg(test)]
    fn memory() -> api::Result<Database> {
        Ok(Database {
            path: PathBuf::new(),
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
        #[allow(clippy::cast_ref_to_mut)]
        let con = unsafe { &mut *(addr_of!(self.con) as *mut Connection) };
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
    type Item = api::Result<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.rows.next() {
            Ok(row) => Some(T::from_row(row?).map_err(Into::into)),
            Err(e) => Some(Err(e.into())),
        }
    }
}
