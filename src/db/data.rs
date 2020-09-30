use crate::api;

/// Iterator over database results.
pub struct DBIter<'a, T> {
    stmt: sqlite::Statement<'a>,
    ty: std::marker::PhantomData<T>,
}

impl<'a, T> DBIter<'a, T> {
    pub fn new(stmt: sqlite::Statement<'a>) -> Self {
        DBIter {
            stmt,
            ty: std::marker::PhantomData,
        }
    }
}

/// Conversion from database entries.
pub trait ReadStmt: Sized {
    type Error: std::fmt::Debug;
    fn read(stmt: &sqlite::Statement) -> Result<Self, Self::Error>;
}

impl<'a, T: ReadStmt> Iterator for DBIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if let Ok(state) = self.stmt.next() {
            if state != sqlite::State::Done {
                match T::read(&self.stmt) {
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

/// Data object for medium.
#[derive(Debug)]
pub struct DBMedium {
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

impl ReadStmt for DBMedium {
    type Error = api::Error;

    fn read(stmt: &sqlite::Statement<'_>) -> api::Result<DBMedium> {
        Ok(DBMedium {
            id: stmt.read(0)?,
            isbn: stmt.read(1)?,
            title: stmt.read(2)?,
            publisher: stmt.read(3)?,
            year: stmt.read(4)?,
            costs: stmt.read(5)?,
            note: stmt.read(6)?,
            borrowable: stmt.read::<i64>(7)? != 0,
            category: stmt.read(8)?,
            authors: stmt
                .read::<String>(9)?
                .split(',')
                .map(|a| a.to_string())
                .collect(),
            borrower: stmt.read(10)?,
            deadline: stmt.read(11)?,
            reservation: stmt.read(12)?,
        })
    }
}

/// Data object for a user.
#[derive(Debug)]
pub struct DBUser {
    pub account: String,
    pub forename: String,
    pub surname: String,
    pub role: String,
    pub may_borrow: bool,
}

impl ReadStmt for DBUser {
    type Error = api::Error;

    fn read(stmt: &sqlite::Statement<'_>) -> api::Result<DBUser> {
        Ok(DBUser {
            account: stmt.read(0)?,
            forename: stmt.read(1)?,
            surname: stmt.read(2)?,
            role: stmt.read(3)?,
            may_borrow: stmt.read::<i64>(4)? != 0,
        })
    }
}

#[derive(Debug)]
pub struct DBCategory {
    pub id: String,
    pub name: String,
    pub section: String,
}

impl ReadStmt for DBCategory {
    type Error = api::Error;

    fn read(stmt: &sqlite::Statement<'_>) -> api::Result<DBCategory> {
        Ok(DBCategory {
            id: stmt.read(0)?,
            name: stmt.read(1)?,
            section: stmt.read(2)?,
        })
    }
}
