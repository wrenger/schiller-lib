use crate::api;

use super::raw::DatabaseExt;
use super::{DBIter, ReadStmt};

// Query
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

pub trait DatabaseUser {
    fn db(&self) -> &sqlite::Connection;

    /// Performes a simple user search with the given `text`.
    fn user_search(&self, text: &str) -> api::Result<DBIter<DBUser>> {
        let mut stmt = self.db().prepare(QUERY_USERS)?;
        stmt.bind(1, text)?;
        stmt.bind(2, text)?;
        stmt.bind(3, text)?;
        stmt.bind(4, text)?;
        Ok(DBIter::new(stmt))
    }

    /// Adds a new user.
    fn user_add(&self, user: &DBUser) -> api::Result<()> {
        let mut stmt = self.db().prepare(ADD_USER)?;
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
    fn user_update(&self, previous_account: &str, user: &DBUser) -> api::Result<()> {
        let transaction = self.db().transaction()?;
        // update user
        let mut stmt = self.db().prepare(UPDATE_USER)?;
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
        let mut stmt = self.db().prepare(UPDATE_USER_BORROWS)?;
        stmt.bind(1, user.account.as_str())?;
        stmt.bind(2, previous_account)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }

        // update reservations
        let mut stmt = self.db().prepare(UPDATE_USER_RESERVATIONS)?;
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
    fn user_delete(&self, account: &str) -> api::Result<()> {
        let transaction = self.db().transaction()?;
        // remove user
        let mut stmt = self.db().prepare(DELETE_USER)?;
        stmt.bind(1, account)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }

        // remove borrows & reservations
        self.db().execute(DELETE_UNUSED_USERS)?;
        transaction.commit()?;
        Ok(())
    }
}