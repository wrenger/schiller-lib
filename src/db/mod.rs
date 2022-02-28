use std::collections::hash_map::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::api;

pub mod book;
pub use book::{Book, BookSearch, BookState};
pub mod category;
pub use category::Category;
pub mod lending;
pub mod raw;
pub mod settings;
pub use settings::Settings;
pub mod stats;
pub use stats::Stats;
pub mod structure;
pub mod user;
pub use user::User;

use raw::StatementExt;

use super::PKG_VERSION;

pub struct Database {
    path: PathBuf,
    con: sqlite::Connection,
}

impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Database {{ {:?} }}", self.path)
    }
}

impl Database {
    /// Creates a new database at the given path.
    pub fn create(path: &str) -> api::Result<Database> {
        let path = PathBuf::from(path);
        if !path.exists() {
            let database = Database {
                con: sqlite::Connection::open_with_flags(
                    &path,
                    sqlite::OpenFlags::new().set_create().set_read_write(),
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
                con: sqlite::Connection::open_with_flags(
                    &path,
                    sqlite::OpenFlags::new().set_read_write(),
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
            con: sqlite::open(":memory:")?,
        })
    }
}

/// Iterator over database results.
pub struct DBIter<'a, T> {
    stmt: sqlite::Statement<'a>,
    columns: HashMap<String, usize>,
    ty: std::marker::PhantomData<T>,
}

impl<'a, T> DBIter<'a, T> {
    pub fn new(stmt: sqlite::Statement<'a>) -> Self {
        let mut iter = DBIter {
            columns: HashMap::new(),
            stmt,
            ty: std::marker::PhantomData,
        };
        iter.columns = iter.stmt.columns();
        iter
    }
}

/// Conversion from database entries.
pub trait ReadStmt: Sized {
    fn read(stmt: &sqlite::Statement, columns: &HashMap<String, usize>) -> api::Result<Self>;
}

impl<'a, T: ReadStmt> Iterator for DBIter<'a, T> {
    type Item = api::Result<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.stmt.next() {
            Ok(sqlite::State::Row) => Some(T::read(&self.stmt, &self.columns)),
            Ok(sqlite::State::Done) => None,
            Err(e) => Some(Err(e.into())),
        }
    }
}
