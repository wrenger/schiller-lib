use std::fmt;
use std::path::PathBuf;

use gdnative::prelude::*;

mod raw;

use crate::api;

const QUERY_MEDIA_LIST: &str = r#"
select
id,
isbn,
title,
publisher,
year,
costs,
note,
borrowable,
category,
ifnull(group_concat(author.name),'') as authors,
borrower,
deadline,
reservation
from medium
left join author on author.medium=id
group by id
having id like '%'||?||'%'
or isbn like '%'||?||'%'
or title like '%'||?||'%'
or publisher like '%'||?||'%'
or note like '%'||?||'%'
or authors like '%'||?||'%'
"#;

const QUERY_USER_LIST: &str = r#"
select
account,
forename,
surname,
role,
may_borrow
from user
where account like '%'||?||'%'
or forename like '%'||?||'%'
or surname like '%'||?||'%'
or role like '%'||?||'%'
"#;

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

    pub fn search_media_basic(&self, text: &str) -> api::Result<DBIter<DBMedium>> {
        let mut stmt = self.db.prepare(QUERY_MEDIA_LIST)?;
        stmt.bind(1, text)?;
        stmt.bind(2, text)?;
        stmt.bind(3, text)?;
        stmt.bind(4, text)?;
        stmt.bind(5, text)?;
        stmt.bind(6, text)?;
        Ok(DBIter::new(stmt))
    }

    pub fn search_user_basic(&self, text: &str) -> api::Result<DBIter<DBUser>> {
        let mut stmt = self.db.prepare(QUERY_USER_LIST)?;
        stmt.bind(1, text)?;
        stmt.bind(2, text)?;
        stmt.bind(3, text)?;
        stmt.bind(4, text)?;
        Ok(DBIter::new(stmt))
    }

    pub fn update_medium(&self, previous_id: &str, medium: &api::Medium) -> api::Result<()> {
        godot_print!("update_medium {} -> {:?}", previous_id, medium);
        Ok(())
    }

    pub fn delete_medium(&self, id: &str) -> api::Result<()> {
        godot_print!("delete_medium {}", id);
        Ok(())
    }

    pub fn update_user(&self, previous_account: &str, user: &api::User) -> api::Result<()> {
        godot_print!("update_user {} -> {:?}", previous_account, user);
        Ok(())
    }

    pub fn delete_user(&self, account: &str) -> api::Result<()> {
        godot_print!("delete_user {}", account);
        Ok(())
    }
}

pub struct DBIter<'a, T> {
    stmt: sqlite::Statement<'a>,
    ty: std::marker::PhantomData<T>,
}

impl<'a, T> DBIter<'a, T> {
    fn new(stmt: sqlite::Statement<'a>) -> Self {
        DBIter {
            stmt,
            ty: std::marker::PhantomData,
        }
    }
}

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
                        godot_print!("SQLError! {:?}", e);
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
