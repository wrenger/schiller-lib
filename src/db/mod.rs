use std::collections::hash_map::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::api;

mod category;
mod medium;
mod raw;
mod rental;
mod user;
mod settings;

pub use category::*;
pub use medium::*;
use raw::StatementExt;
pub use rental::*;
pub use user::*;
pub use settings::*;

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
    /// Opens a database connection to the given project database.
    pub fn new(path: &str) -> Result<Database, api::Error> {
        let path = PathBuf::from(path);
        if path.exists() {
            Ok(Database {
                db: sqlite::Connection::open(&path).map_err(|_| api::Error::FileOpenError)?,
                path,
            })
        } else {
            Err(api::Error::FileNotFound)
        }
    }

    /// Returns the filepath to this database.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl DatabaseCategory for Database {
    fn db(&self) -> &sqlite::Connection {
        &self.db
    }
}

impl DatabaseMedium for Database {
    fn db(&self) -> &sqlite::Connection {
        &self.db
    }
}

impl DatabaseRental for Database {
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
    fn read(stmt: &sqlite::Statement, columns: &HashMap<String, usize>) -> Result<Self, Self::Error>;
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
