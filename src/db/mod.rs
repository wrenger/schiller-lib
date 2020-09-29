use std::fmt;
use std::path::PathBuf;

use gdnative::prelude::*;

mod raw;
use crate::api;
use raw::DatabaseExt;

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

const UPDATE_MEDIUM: &str = r#"
update medium set id=?, isbn=?, title=?, publisher=?, year=?, costs=?, note=?, borrowable=?, category=? where id=?
"#;
const UPDATE_MEDIUM_AUTHORS: &str = r#"
update author set medium=? where medium=?
"#;

const DELETE_MEDIUM: &str = r#"
delete from medium where id=?
"#;
const DELETE_UNUSED_AUTHORS: &str = r#"
delete from author where medium not in (select id from medium)
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

const UPDATE_USER: &str = r#"
update user set account=?, forename=?, surname=?, role=?, may_borrow=? where account=?
"#;
const UPDATE_USER_BORROWS: &str = r#"
update medium set borrower=? where borrower=?;
"#;
const UPDATE_USER_RESERVATIONS: &str = r#"
update medium set reservation=? where reservation=?;
"#;

const DELETE_USER: &str = r#"
delete from user where account=?
"#;
const DELETE_UNUSED_USERS: &str = r#"
update medium set reservation='' where reservation not in (select account from user);
update medium set borrower='' where reservation not in (select account from user);
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

    pub fn update_medium(&self, previous_id: &str, medium: &DBMedium) -> api::Result<()> {
        godot_print!("update_medium {} -> {:?}", previous_id, medium);
        let transaction = self.db.transaction()?;
        // update medium
        let mut stmt = self.db.prepare(UPDATE_MEDIUM)?;
        stmt.bind(1, medium.id.as_str())?;
        stmt.bind(2, medium.isbn.as_str())?;
        stmt.bind(3, medium.title.as_str())?;
        stmt.bind(4, medium.publisher.as_str())?;
        stmt.bind(5, medium.year)?;
        stmt.bind(6, medium.costs)?;
        stmt.bind(7, medium.note.as_str())?;
        stmt.bind(8, medium.borrowable as i64)?;
        stmt.bind(9, medium.category.as_str())?;
        stmt.bind(10, previous_id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }

        // update authors
        let mut stmt = self.db.prepare(UPDATE_MEDIUM_AUTHORS)?;
        stmt.bind(1, medium.id.as_str())?;
        stmt.bind(2, previous_id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        transaction.commit()?;
        Ok(())
    }

    pub fn delete_medium(&self, id: &str) -> api::Result<()> {
        godot_print!("delete_medium {}", id);
        let transaction = self.db.transaction()?;
        // delete medium
        let mut stmt = self.db.prepare(DELETE_MEDIUM)?;
        stmt.bind(1, id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }

        // delete missing authors
        self.db.execute(DELETE_UNUSED_AUTHORS)?;
        transaction.commit()?;
        Ok(())
    }

    pub fn update_user(&self, previous_account: &str, user: &DBUser) -> api::Result<()> {
        godot_print!("update_user {} -> {:?}", previous_account, user);
        let transaction = self.db.transaction()?;
        // update user
        let mut stmt = self.db.prepare(UPDATE_USER)?;
        stmt.bind(1, user.account.as_str())?;
        stmt.bind(2, user.forename.as_str())?;
        stmt.bind(3, user.surname.as_str())?;
        stmt.bind(4, user.role.as_str())?;
        stmt.bind(5, user.may_borrow as i64)?;
        stmt.bind(6, previous_account)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }

        // update borrows
        let mut stmt = self.db.prepare(UPDATE_USER_BORROWS)?;
        stmt.bind(1, user.account.as_str())?;
        stmt.bind(2, previous_account)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }

        // update reservations
        let mut stmt = self.db.prepare(UPDATE_USER_RESERVATIONS)?;
        stmt.bind(1, user.account.as_str())?;
        stmt.bind(2, previous_account)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        transaction.commit()?;
        Ok(())
    }

    pub fn delete_user(&self, account: &str) -> api::Result<()> {
        godot_print!("delete_user {}", account);
        let transaction = self.db.transaction()?;
        // remove user
        let mut stmt = self.db.prepare(DELETE_USER)?;
        stmt.bind(1, account)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }

        // remove borrows & reservations
        self.db.execute(DELETE_UNUSED_USERS)?;
        transaction.commit()?;
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
