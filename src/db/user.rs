use crate::api;

use super::{DBIter, Database, FromRow};

// Query
const FETCH_USER: &str = "\
    select \
    account, \
    forename, \
    surname, \
    role, \
    may_borrow \
    from user \
    where account=? \
";

const QUERY_USERS: &str = "\
    select \
    account, \
    forename, \
    surname, \
    role, \
    may_borrow \
    \
    from user \
    where account like '%'||?1||'%' \
    or forename like '%'||?1||'%' \
    or surname like '%'||?1||'%' \
    or role like '%'||?1||'%' \
    order by account \
";

const ADD_USER: &str = "\
    insert into user values (?, ?, ?, ?, ?) \
";
const UPDATE_USER: &str = "\
    update user set account=?, forename=?, surname=?, role=?, may_borrow=? where account=? \
";
const UPDATE_USER_BORROWS: &str = "
    update medium set borrower=? where borrower=? \
";
const UPDATE_USER_RESERVATIONS: &str = "\
    update medium set reservation=? where reservation=? \
";

const DELETE_USER: &str = "\
    delete from user where account=? \
";
const DELETE_UNUSED_USERS: &str = "\
    update medium set reservation='' where reservation not in (select account from user); \
    update medium set borrower='' where borrower not in (select account from user); \
";
const DELETE_USER_ROLES: &str = "\
    update user set role='' \
";
const UPDATE_USER_ROLE: &str = "\
    update user set role=? where account=? \
";

/// Data object for a user.
#[derive(Debug, Clone, gdnative::ToVariant, gdnative::FromVariant)]
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
        !self.account.trim().is_empty()
            && !self.forename.trim().is_empty()
            && !self.surname.trim().is_empty()
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
pub fn fetch(db: &Database, id: &str) -> api::Result<User> {
    Ok(db.con.query_row(FETCH_USER, [id], User::from_row)?)
}

/// Performes a simple user search with the given `text`.
pub fn search<'a>(db: &'a Database, text: &str) -> api::Result<Vec<User>> {
    let mut stmt = db.con.prepare(QUERY_USERS)?;
    let rows = stmt.query([text.trim()])?;
    DBIter::new(rows).collect()
}

/// Adds a new user.
pub fn add(db: &Database, user: &User) -> api::Result<()> {
    if !user.is_valid() {
        return Err(api::Error::InvalidUser);
    }
    db.con.execute(
        ADD_USER,
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
pub fn update(db: &Database, previous_account: &str, user: &User) -> api::Result<()> {
    let previous_account = previous_account.trim();
    if previous_account.is_empty() || !user.is_valid() {
        return Err(api::Error::InvalidUser);
    }
    let transaction = db.transaction()?;
    // update user
    transaction.execute(
        UPDATE_USER,
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
    transaction.execute(UPDATE_USER_BORROWS, [user.account.trim(), previous_account])?;

    // update reservations
    transaction.execute(
        UPDATE_USER_RESERVATIONS,
        [user.account.trim(), previous_account],
    )?;
    transaction.commit()?;
    Ok(())
}

/// Deletes the user.
/// This includes all its borrows & reservations.
pub fn delete(db: &Database, account: &str) -> api::Result<()> {
    let account = account.trim();
    if account.is_empty() {
        return Err(api::Error::InvalidUser);
    }
    let transaction = db.transaction()?;
    // remove user
    transaction.execute(DELETE_USER, [account])?;

    // remove borrows & reservations
    transaction.execute(DELETE_UNUSED_USERS, [])?;
    transaction.commit()?;
    Ok(())
}

/// Deletes the roles from all users and inserts the new roles.
///
/// The roles of all users not contained in the given list are cleared.
pub fn update_roles(db: &Database, users: &[(&str, &str)]) -> api::Result<()> {
    let transaction = db.transaction()?;
    transaction.execute(DELETE_USER_ROLES, [])?;

    let mut stmt = transaction.prepare(UPDATE_USER_ROLE)?;
    for &(account, role) in users {
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

        let result = user::search(&db, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], user);

        user::update(
            &db,
            &user.account,
            &User {
                role: "Teacher".into(),
                ..user.clone()
            },
        )
        .unwrap();
        let result = user::search(&db, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].role, "Teacher");

        user::delete(&db, &user.account).unwrap();
        let result = user::search(&db, "").unwrap();
        assert_eq!(result.len(), 0);
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

        let result = user::search(&db, "").unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], user2);
        assert_eq!(result[1], user1);

        user::update_roles(&db, &[("foo.bar", "Teacher")]).unwrap();

        user1.role = "Teacher".into();
        user2.role = "".into();

        let result = user::search(&db, "").unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], user2);
        assert_eq!(result[1], user1);
    }
}
