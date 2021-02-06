use std::collections::hash_map::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::api;

mod book;
mod category;
mod lending;
mod raw;
mod settings;
mod structure;
mod user;
mod stats;

pub use book::*;
pub use category::*;
pub use lending::*;
use raw::StatementExt;
pub use settings::*;
pub use structure::*;
pub use user::*;
pub use stats::*;

use super::PKG_VERSION;

pub struct Database {
    path: PathBuf,
    db: sqlite::Connection,
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
                db: sqlite::Connection::open_with_flags(
                    &path,
                    sqlite::OpenFlags::new().set_create().set_read_write(),
                )
                .map_err(|_| api::Error::FileOpenError)?,
                path,
            };
            database.structure_create(PKG_VERSION)?;
            Ok(database)
        } else {
            Err(api::Error::FileOpenError)
        }
    }

    /// Opens a database connection to the given project database.
    pub fn open(path: &str) -> api::Result<(Database, bool)> {
        let path = PathBuf::from(path);
        if path.exists() {
            let database = Database {
                db: sqlite::Connection::open_with_flags(
                    &path,
                    sqlite::OpenFlags::new().set_read_write(),
                )
                .map_err(|_| api::Error::FileOpenError)?,
                path,
            };
            let updated = database.structure_migrate(PKG_VERSION)?;
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
            db: sqlite::open(":memory:")?,
        })
    }
}

impl DatabaseCategory for Database {
    fn db(&self) -> &sqlite::Connection {
        &self.db
    }
}

impl DatabaseBook for Database {
    fn db(&self) -> &sqlite::Connection {
        &self.db
    }
}

impl DatabaseLending for Database {
    fn db(&self) -> &sqlite::Connection {
        &self.db
    }
}

impl DatabaseUser for Database {
    fn db(&self) -> &sqlite::Connection {
        &self.db
    }
}

impl DatabaseSettings for Database {
    fn db(&self) -> &sqlite::Connection {
        &self.db
    }
}

impl DatabaseStats for Database {
    fn db(&self) -> &sqlite::Connection {
        &self.db
    }
}

impl DatabaseStructure for Database {}

/// Iterator over database results.
pub struct DBIter<'a, T> {
    stmt: sqlite::Statement<'a>,
    columns: HashMap<String, usize>,
    ty: std::marker::PhantomData<T>,
}

impl<'a, T> DBIter<'a, T> {
    pub fn new(stmt: sqlite::Statement<'a>) -> Self {
        DBIter {
            columns: stmt.columns(),
            stmt,
            ty: std::marker::PhantomData,
        }
    }
}

/// Conversion from database entries.
pub trait ReadStmt: Sized {
    type Error: std::fmt::Debug;
    fn read(
        stmt: &sqlite::Statement,
        columns: &HashMap<String, usize>,
    ) -> Result<Self, Self::Error>;
}

impl<'a, T: ReadStmt> Iterator for DBIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if let Ok(state) = self.stmt.next() {
            if state != sqlite::State::Done {
                match T::read(&self.stmt, &self.columns) {
                    Ok(r) => Some(r),
                    Err(e) => {
                        gdnative::godot_print!("SQLError! {:?}", e);
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
