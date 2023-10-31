use super::{book, user};
use super::{Book, DBIter, Database, FromRow, User};
use crate::error::{Error, Result};

/// Lends the book to the specified user.
pub fn lend(db: &Database, id: &str, account: &str, deadline: &str) -> Result<Book> {
    let mut book = book::fetch(db, id)?;
    let user = user::fetch(db, account)?;

    if !user.may_borrow {
        return Err(Error::LendingUserMayNotBorrow);
    }
    if !book.borrowable {
        return Err(Error::LendingBookNotBorrowable);
    }
    // Allow renewal
    if !book.borrower.is_empty() && book.borrower != user.account {
        return Err(Error::LendingBookAlreadyBorrowed);
    }
    if !book.reservation.is_empty() {
        if book.reservation == user.account {
            release(db, id)?; // Allow lending to reserver
        } else {
            return Err(Error::LendingBookAlreadyReserved);
        }
    }

    db.con.execute(
        "update medium set borrower=?, deadline=? where id=?",
        [user.account.trim(), deadline.trim(), book.id.trim()],
    )?;

    book.borrower = user.account.clone();
    book.deadline = deadline.trim().into();
    Ok(book)
}

/// Returns the book.
pub fn return_back(db: &Database, id: &str) -> Result<Book> {
    let mut book = book::fetch(db, id)?;

    if book.borrower.is_empty() {
        return Err(Error::Logic);
    }

    db.con.execute(
        "update medium set borrower='', deadline='' where id=?",
        [book.id.trim()],
    )?;
    book.borrower = String::new();
    book.deadline = String::new();
    Ok(book)
}

/// Creates a reservation for the borrowed book.
pub fn reserve(db: &Database, id: &str, account: &str) -> Result<Book> {
    let mut book = book::fetch(db, id)?;
    let user = user::fetch(db, account)?;

    if !user.may_borrow {
        return Err(Error::LendingUserMayNotBorrow);
    }
    if !book.borrowable {
        return Err(Error::LendingBookNotBorrowable);
    }
    if !book.reservation.is_empty() {
        return Err(Error::LendingBookAlreadyReserved);
    }
    if book.borrower.is_empty() {
        return Err(Error::LendingBookNotBorrowed);
    }
    if book.borrower == user.account {
        return Err(Error::LendingBookAlreadyBorrowedByUser);
    }

    db.con.execute(
        "update medium set reservation=? where id=?",
        [user.account.trim(), book.id.trim()],
    )?;
    book.reservation = user.account.clone();
    Ok(book)
}

/// Removes the reservation from the specified book.
pub fn release(db: &Database, id: &str) -> Result<Book> {
    let mut book = book::fetch(db, id)?;

    if book.reservation.is_empty() {
        return Err(Error::Logic);
    }

    db.con.execute(
        "update medium set reservation='' where id=?",
        [book.id.trim()],
    )?;
    book.reservation = String::new();
    Ok(book)
}

/// Return the list of currently all borrowed books.
pub fn borrowed(db: &Database) -> Result<Vec<(Book, User)>> {
    const QUERY_CURRENT: &str = "\
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
        JulianDay(date(deadline)) - JulianDay(date('now')) as days \
        \
        from medium \
        left join author on author.medium=id \
        join user on account=borrower \
        group by id \
        order by days \
    ";
    let mut stmt = db.con.prepare(QUERY_CURRENT)?;
    let rows = stmt.query([])?;
    DBIter::new(rows).collect()
}

/// Return the list of expired loan periods.
pub fn overdues(db: &Database) -> Result<Vec<(Book, User)>> {
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
        order by days \
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
