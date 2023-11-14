use crate::{
    error::{Error, Result},
    mail::account_is_valid,
};

use super::{collect_rows, Database, FromRow};

use serde::{Deserialize, Serialize};

/// Data object for a user.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct User {
    pub account: String,
    pub forename: String,
    pub surname: String,
    pub role: String,
    pub may_borrow: bool,
}

impl User {
    fn is_valid(&self) -> bool {
        account_is_valid(self.account.trim())
            && !self.forename.trim().is_empty()
            && !self.surname.trim().is_empty()
            && !self.role.trim().is_empty()
    }
}

impl FromRow for User {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<User> {
        Ok(User {
            account: row.get("account")?,
            forename: row.get("forename")?,
            surname: row.get("surname")?,
            role: row.get("role")?,
            may_borrow: row.get("may_borrow")?,
        })
    }
}

/// Returns the user with the given `id`.
pub fn fetch(db: &Database, id: &str) -> Result<User> {
    Ok(db.con.query_row(
        "select \
        account, \
        forename, \
        surname, \
        role, \
        may_borrow \
        from user \
        where account=?",
        [id],
        User::from_row,
    )?)
}

/// Parameters for the normal search
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct UserSearch {
    pub query: String,
    pub may_borrow: Option<bool>,
    pub offset: usize,
    pub limit: usize,
}

impl Default for UserSearch {
    fn default() -> Self {
        Self {
            query: Default::default(),
            may_borrow: None,
            offset: 0,
            limit: 100,
        }
    }
}

/// Performes a simple user search with the given `text`.
pub fn search(db: &Database, params: &UserSearch) -> Result<(usize, Vec<User>)> {
    let mut stmt = db.con.prepare(
        "select \
        account, \
        forename, \
        surname, \
        role, \
        may_borrow, \
        count(*) over() as total_count \
        \
        from user \
        where (account like '%'||?1||'%' \
        or forename like '%'||?1||'%' \
        or surname like '%'||?1||'%' \
        or role like '%'||?1||'%') \
        and may_borrow like '%'||?2||'%' \
        order by \
            case \
                when account like ?1||'%' then 0 \
                else 1 \
            end asc, \
            account asc \
        limit ?3 offset ?4",
    )?;
    let rows = stmt.query(rusqlite::params![
        &params.query.trim(),
        match params.may_borrow {
            Some(true) => "1",
            Some(false) => "0",
            None => "%",
        },
        params.limit,
        params.offset
    ])?;

    collect_rows(rows)
}

/// Adds a new user.
pub fn add(db: &Database, user: &User) -> Result<()> {
    if !user.is_valid() {
        return Err(Error::InvalidUser);
    }
    db.con.execute(
        "insert into user values (?, ?, ?, ?, ?)",
        rusqlite::params![
            user.account.trim(),
            user.forename.trim(),
            user.surname.trim(),
            user.role.trim(),
            user.may_borrow as i64,
        ],
    )?;
    Ok(())
}

/// Updates the user and all references if its account changes.
pub fn update(db: &Database, previous_account: &str, user: &User) -> Result<()> {
    let previous_account = previous_account.trim();
    if previous_account.is_empty() || !user.is_valid() {
        return Err(Error::InvalidUser);
    }
    let transaction = db.transaction()?;
    // update user
    transaction.execute(
        "update user set account=?, forename=?, surname=?, role=?, may_borrow=? where account=?",
        rusqlite::params![
            user.account.trim(),
            user.forename.trim(),
            user.surname.trim(),
            user.role.trim(),
            user.may_borrow as i64,
            previous_account,
        ],
    )?;

    // update borrows
    transaction.execute(
        "update medium set borrower=? where borrower=?",
        [user.account.trim(), previous_account],
    )?;

    // update reservations
    transaction.execute(
        "update medium set reservation=? where reservation=?",
        [user.account.trim(), previous_account],
    )?;
    transaction.commit()?;
    Ok(())
}

/// Deletes the user.
/// This includes all its borrows & reservations.
pub fn delete(db: &Database, account: &str) -> Result<()> {
    let account = account.trim();
    if account.is_empty() {
        return Err(Error::InvalidUser);
    }
    let transaction = db.transaction()?;
    // remove user
    transaction.execute("delete from user where account=?", [account])?;

    // remove borrows & reservations
    transaction.execute(
        "update medium set reservation='' \
        where reservation not in (select account from user); \
        update medium set borrower='' \
        where borrower not in (select account from user);",
        [],
    )?;
    transaction.commit()?;
    Ok(())
}

/// Deletes the roles from all users and inserts the new roles.
///
/// The roles of all users not contained in the given list are cleared.
pub fn update_roles(db: &Database, users: &[(String, String)]) -> Result<()> {
    let transaction = db.transaction()?;
    transaction.execute("update user set role='-'", [])?;

    let mut stmt = transaction.prepare("update user set role=? where account=?")?;
    for (account, role) in users {
        let account = account.trim();
        if !account.is_empty() {
            stmt.execute([role.trim(), account])?;
        }
    }
    drop(stmt);
    transaction.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn add_update_remove_users() {
        let db = Database::memory().unwrap();
        structure::create(&db, PKG_VERSION).unwrap();

        let user = User {
            account: "foo.bar".into(),
            forename: "Foo".into(),
            surname: "Bar".into(),
            role: "Demo".into(),
            may_borrow: true,
        };
        user::add(&db, &user).unwrap();

        let (count, users) = user::search(
            &db,
            &UserSearch {
                query: "".to_owned(),
                may_borrow: None,
                offset: 0,
                limit: 100,
            },
        )
        .unwrap();
        assert_eq!(count, 1);
        assert_eq!(users[0], user);

        user::update(
            &db,
            &user.account,
            &User {
                role: "Teacher".into(),
                ..user.clone()
            },
        )
        .unwrap();
        let (count, users) = user::search(
            &db,
            &UserSearch {
                query: "".to_owned(),
                may_borrow: None,
                offset: 0,
                limit: 100,
            },
        )
        .unwrap();
        assert_eq!(count, 1);
        assert_eq!(users[0].role, "Teacher");

        user::delete(&db, &user.account).unwrap();
        let (count, _) = user::search(
            &db,
            &UserSearch {
                query: "".to_owned(),
                may_borrow: None,
                offset: 0,
                limit: 100,
            },
        )
        .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn update_user_roles() {
        let db = Database::memory().unwrap();
        structure::create(&db, PKG_VERSION).unwrap();

        let mut user1 = User {
            account: "foo.bar".into(),
            forename: "Foo".into(),
            surname: "Bar".into(),
            role: "Demo".into(),
            may_borrow: true,
        };
        let mut user2 = User {
            account: "baz.boz".into(),
            forename: "Baz".into(),
            surname: "Boz".into(),
            role: "Demo".into(),
            may_borrow: true,
        };
        user::add(&db, &user1).unwrap();
        user::add(&db, &user2).unwrap();

        let (count, users) = user::search(
            &db,
            &UserSearch {
                query: "".to_owned(),
                may_borrow: None,
                offset: 0,
                limit: 100,
            },
        )
        .unwrap();
        assert_eq!(count, 2);
        assert_eq!(users[0], user2);
        assert_eq!(users[1], user1);

        user::update_roles(&db, &[("foo.bar".into(), "Teacher".into())]).unwrap();

        user1.role = "Teacher".into();
        user2.role = "-".into();

        let (count, users) = user::search(
            &db,
            &UserSearch {
                query: "".to_owned(),
                may_borrow: None,
                offset: 0,
                limit: 100,
            },
        )
        .unwrap();
        assert_eq!(count, 2);
        assert_eq!(users[0], user2);
        assert_eq!(users[1], user1);
    }
}
