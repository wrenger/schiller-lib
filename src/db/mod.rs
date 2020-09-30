use std::fmt;
use std::path::PathBuf;

use crate::api;

mod data;
pub use data::{DBCategory, DBIter, DBMedium, DBUser};
mod raw;
use raw::DatabaseExt;

// Query

const QUERY_MEDIA: &str = r#"
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

const QUERY_USERS: &str = r#"
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

const QUERY_CATEGORIES: &str = r#"
select * from category order by section, id
"#;

// Update

// Medium

const ADD_MEDIUM: &str = r#"
insert into medium values (?, ?, ?, ?, ?, ?, ?, ?, ?, '', '', '')
"#;

const ADD_MEDIUM_AUTHOR: &str = r#"
insert or ignore into author values (?, ?)
"#;
const UPDATE_MEDIUM: &str = r#"
update medium set id=?, isbn=?, title=?, publisher=?, year=?, costs=?, note=?, borrowable=?, category=? where id=?
"#;
const UPDATE_MEDIUM_AUTHORS: &str = r#"
update author set medium=? where medium=?
"#;
const UPDATE_LEND: &str = r#"
update medium set borrower=?, deadline=? where id=?
"#;
const UPDATE_REVOKE: &str = r#"
update medium set borrower='', deadline='' where id=?
"#;
const UPDATE_RESERVE: &str = r#"
update medium set reservation=? where id=?
"#;
const UPDATE_RELEASE: &str = r#"
update medium set reservation='' where id=?
"#;

const DELETE_MEDIUM: &str = r#"
delete from medium where id=?
"#;
const DELETE_UNUSED_AUTHORS: &str = r#"
delete from author where medium not in (select id from medium)
"#;

// User

const ADD_USER: &str = r#"
insert into user values (?, ?, ?, ?, ?)
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

    /// Performes a simple media search with the given `text`.
    pub fn search_media(&self, text: &str) -> api::Result<DBIter<DBMedium>> {
        let mut stmt = self.db.prepare(QUERY_MEDIA)?;
        stmt.bind(1, text)?;
        stmt.bind(2, text)?;
        stmt.bind(3, text)?;
        stmt.bind(4, text)?;
        stmt.bind(5, text)?;
        stmt.bind(6, text)?;
        Ok(DBIter::new(stmt))
    }

    /// Performes a simple user search with the given `text`.
    pub fn search_users(&self, text: &str) -> api::Result<DBIter<DBUser>> {
        let mut stmt = self.db.prepare(QUERY_USERS)?;
        stmt.bind(1, text)?;
        stmt.bind(2, text)?;
        stmt.bind(3, text)?;
        stmt.bind(4, text)?;
        Ok(DBIter::new(stmt))
    }

    /// Performes a simple user search with the given `text`.
    pub fn categories(&self) -> api::Result<DBIter<DBCategory>> {
        let stmt = self.db.prepare(QUERY_CATEGORIES)?;
        Ok(DBIter::new(stmt))
    }

    /// Adds a new medium.
    pub fn add_medium(&self, medium: &DBMedium) -> api::Result<()> {
        // Add medium
        let transaction = self.db.transaction()?;
        let mut stmt = self.db.prepare(ADD_MEDIUM)?;
        stmt.bind(1, medium.id.as_str())?;
        stmt.bind(2, medium.isbn.as_str())?;
        stmt.bind(3, medium.title.as_str())?;
        stmt.bind(4, medium.publisher.as_str())?;
        stmt.bind(5, medium.year)?;
        stmt.bind(6, medium.costs)?;
        stmt.bind(7, medium.note.as_str())?;
        stmt.bind(8, medium.borrowable as i64)?;
        stmt.bind(9, medium.category.as_str())?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        // Add authors
        for author in &medium.authors {
            let mut stmt = self.db.prepare(ADD_MEDIUM_AUTHOR)?;
            stmt.bind(1, author.as_str())?;
            stmt.bind(2, medium.id.as_str())?;
            if stmt.next()? != sqlite::State::Done {
                return Err(api::Error::SQLError);
            }
        }
        transaction.commit()?;
        Ok(())
    }

    /// Updates the medium and all references if its id changes.
    pub fn update_medium(&self, previous_id: &str, medium: &DBMedium) -> api::Result<()> {
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

    /// Deletes the medium including the its authors.
    /// Also borrowers & reservations for this medium are removed.
    pub fn delete_medium(&self, id: &str) -> api::Result<()> {
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

    /// Adds a new user.
    pub fn add_user(&self, user: &DBUser) -> api::Result<()> {
        let mut stmt = self.db.prepare(ADD_USER)?;
        stmt.bind(1, user.account.as_str())?;
        stmt.bind(2, user.forename.as_str())?;
        stmt.bind(3, user.surname.as_str())?;
        stmt.bind(4, user.role.as_str())?;
        stmt.bind(5, user.may_borrow as i64)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        Ok(())
    }

    /// Updates the user and all references if its account changes.
    pub fn update_user(&self, previous_account: &str, user: &DBUser) -> api::Result<()> {
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

    /// Deletes the user.
    /// This includes all its borrows & reservations.
    pub fn delete_user(&self, account: &str) -> api::Result<()> {
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

    /// Lends the medium to the specified user.
    pub fn lend(&self, id: &str, account: &str, deadline: &str) -> api::Result<()> {
        let mut stmt = self.db.prepare(UPDATE_LEND)?;
        stmt.bind(1, account)?;
        stmt.bind(2, deadline)?;
        stmt.bind(3, id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        Ok(())
    }

    /// Revokes the borrowing when a borrowed medium is returned.
    pub fn revoke(&self, id: &str) -> api::Result<()> {
        let mut stmt = self.db.prepare(UPDATE_REVOKE)?;
        stmt.bind(1, id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        Ok(())
    }

    /// Creates a reservation for the borrowed medium.
    pub fn reserve(&self, id: &str, account: &str) -> api::Result<()> {
        let mut stmt = self.db.prepare(UPDATE_RESERVE)?;
        stmt.bind(1, account)?;
        stmt.bind(2, id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        Ok(())
    }

    /// Removes the reservation from the specified medium.
    pub fn release(&self, id: &str) -> api::Result<()> {
        let mut stmt = self.db.prepare(UPDATE_RELEASE)?;
        stmt.bind(1, id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        Ok(())
    }
}
