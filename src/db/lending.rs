use std::collections::HashMap;

use super::{Book, DBIter, Database, ReadStmt, User};
use crate::api;

const UPDATE_LEND: &str = "\
    update medium set borrower=?, deadline=? where id=? \
";
const UPDATE_REVOKE: &str = "\
    update medium set borrower='', deadline='' where id=? \
";
const UPDATE_RESERVE: &str = "\
    update medium set reservation=? where id=? \
";
const UPDATE_RELEASE: &str = "\
    update medium set reservation='' where id=? \
";
const QUERY_EXPIRED: &str = "\
    select \
    id, \
    isbn, \
    title, \
    publisher, \
    year, \
    costs, \
    note, \
    borrowable, \
    category, \
    ifnull(group_concat(author.name),'') as authors, \
    borrower, \
    deadline, \
    reservation, \

    account, \
    forename, \
    surname, \
    role, \
    may_borrow, \

    JulianDay(date('now')) - JulianDay(date(deadline)) as days \

    from medium \
    left join author on author.medium=id \
    join user on account=borrower \
    where days > 0 \
    group by id \
    order by role, account \
";

/// Lends the book to the specified user.
pub fn lend(db: &Database, book: &mut Book, user: &User, days: i64) -> api::Result<()> {
    if !user.may_borrow {
        return Err(api::Error::LendingUserMayNotBorrow);
    }
    if !book.borrowable {
        return Err(api::Error::LendingBookNotBorrowable);
    }
    if !book.reservation.is_empty() {
        if book.reservation == user.account {
            release(db, book)?; // Allow lending to reserver
        } else {
            return Err(api::Error::LendingBookAlreadyReserved);
        }
    }
    // Allow renewal
    if !book.borrower.is_empty() && book.borrower != user.account {
        return Err(api::Error::LendingBookAlreadyBorrowed);
    }

    let deadline = chrono::Utc::today() + chrono::Duration::days(days);
    let deadline = deadline.format("%F").to_string();
    gdnative::godot_print!(
        "Lend {} to {} deadline {}",
        &book.id,
        &user.account,
        &deadline
    );

    let mut stmt = db.db.prepare(UPDATE_LEND)?;
    stmt.bind(1, user.account.as_str())?;
    stmt.bind(2, deadline.as_str())?;
    stmt.bind(3, book.id.as_str())?;
    if stmt.next()? != sqlite::State::Done {
        return Err(api::Error::SQLError);
    }

    book.borrower = user.account.clone();
    book.deadline = deadline;
    Ok(())
}

/// Returns the book.
pub fn return_back(db: &Database, book: &mut Book) -> api::Result<()> {
    if book.borrower.is_empty() {
        return Err(api::Error::LogicError);
    }

    let mut stmt = db.db.prepare(UPDATE_REVOKE)?;
    stmt.bind(1, book.id.as_str())?;
    if stmt.next()? != sqlite::State::Done {
        return Err(api::Error::SQLError);
    }
    book.borrower = String::new();
    book.deadline = String::new();
    Ok(())
}

/// Creates a reservation for the borrowed book.
pub fn reserve(db: &Database, book: &mut Book, user: &User) -> api::Result<()> {
    if !user.may_borrow {
        return Err(api::Error::LendingUserMayNotBorrow);
    }
    if !book.borrowable {
        return Err(api::Error::LendingBookNotBorrowable);
    }
    if !book.reservation.is_empty() {
        return Err(api::Error::LendingBookAlreadyReserved);
    }
    if book.borrower.is_empty() {
        return Err(api::Error::LendingBookNotBorrowed);
    }
    if book.borrower == user.account {
        return Err(api::Error::LendingBookAlreadyBorrowedByUser);
    }

    let mut stmt = db.db.prepare(UPDATE_RESERVE)?;
    stmt.bind(1, user.account.as_str())?;
    stmt.bind(2, book.id.as_str())?;
    if stmt.next()? != sqlite::State::Done {
        return Err(api::Error::SQLError);
    }
    book.reservation = user.account.clone();
    Ok(())
}

/// Removes the reservation from the specified book.
pub fn release(db: &Database, book: &mut Book) -> api::Result<()> {
    if book.reservation.is_empty() {
        return Err(api::Error::LogicError);
    }

    let mut stmt = db.db.prepare(UPDATE_RELEASE)?;
    stmt.bind(1, book.id.as_str())?;
    if stmt.next()? != sqlite::State::Done {
        return Err(api::Error::SQLError);
    }
    book.reservation = String::new();
    Ok(())
}

/// Return the list of expired loan periods.
pub fn overdues(db: &Database) -> api::Result<DBIter<(Book, User)>> {
    let stmt = db.db.prepare(QUERY_EXPIRED)?;
    Ok(DBIter::new(stmt))
}

impl ReadStmt for (Book, User) {
    type Error = api::Error;

    fn read(
        stmt: &sqlite::Statement<'_>,
        columns: &HashMap<String, usize>,
    ) -> api::Result<(Book, User)> {
        Ok((Book::read(stmt, columns)?, User::read(stmt, columns)?))
    }
}
