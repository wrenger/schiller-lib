use super::{Book, DBIter, Database, FromRow, User};
use crate::api;

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

    db.con.execute(
        "update medium set borrower=?, deadline=? where id=?",
        [user.account.trim(), deadline.trim(), book.id.trim()],
    )?;

    book.borrower = user.account.clone();
    book.deadline = deadline;
    Ok(())
}

/// Returns the book.
pub fn return_back(db: &Database, book: &mut Book) -> api::Result<()> {
    if book.borrower.is_empty() {
        return Err(api::Error::Logic);
    }

    db.con.execute(
        "update medium set borrower='', deadline='' where id=?",
        [book.id.trim()],
    )?;
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

    db.con.execute(
        "update medium set reservation=? where id=?",
        [user.account.trim(), book.id.trim()],
    )?;
    book.reservation = user.account.clone();
    Ok(())
}

/// Removes the reservation from the specified book.
pub fn release(db: &Database, book: &mut Book) -> api::Result<()> {
    if book.reservation.is_empty() {
        return Err(api::Error::Logic);
    }

    db.con.execute(
        "update medium set reservation='' where id=?",
        [book.id.trim()],
    )?;
    book.reservation = String::new();
    Ok(())
}

/// Return the list of expired loan periods.
pub fn overdues(db: &Database) -> api::Result<Vec<(Book, User)>> {
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
        \
        account, \
        forename, \
        surname, \
        role, \
        may_borrow, \
        \
        JulianDay(date('now')) - JulianDay(date(deadline)) as days \
        \
        from medium \
        left join author on author.medium=id \
        join user on account=borrower \
        where days > 0 \
        group by id \
        order by role, account \
    ";
    let mut stmt = db.con.prepare(QUERY_EXPIRED)?;
    let rows = stmt.query([])?;
    DBIter::new(rows).collect()
}

impl FromRow for (Book, User) {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<(Book, User)> {
        Ok((Book::from_row(row)?, User::from_row(row)?))
    }
}
