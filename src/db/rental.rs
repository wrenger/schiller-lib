use std::collections::HashMap;

use super::{DBIter, Medium, User, ReadStmt};
use crate::api;

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

const QUERY_OVERDUES: &str = r#"
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
reservation,

account,
forename,
surname,
role,
may_borrow,

JulianDay(date('now')) - JulianDay(date(deadline)) as days

from Medium
left join author on author.medium=id
join user on account=borrower
where days > 0
group by id
order by role, account
"#;

pub trait DatabaseRental {
    fn db(&self) -> &sqlite::Connection;

    /// Lends the medium to the specified user.
    fn rental_lend(&self, medium: &mut Medium, user: &User, days: i64) -> api::Result<()> {
        if !user.may_borrow {
            return Err(api::Error::RentalUserMayNotBorrow);
        }
        if !medium.borrowable {
            return Err(api::Error::RentalMediumNotBorrowable);
        }
        if !medium.borrower.is_empty() {
            return Err(api::Error::RentalMediumAlreadyBorrowed);
        }
        if !medium.reservation.is_empty() {
            if medium.reservation == user.account {
                self.rental_release(medium)?; // Allow lending to reserver
            } else {
                return Err(api::Error::RentalMediumAlreadyReserved);
            }
        }

        let deadline = chrono::Utc::today() + chrono::Duration::days(days);
        let deadline = deadline.format("%F").to_string();
        gdnative::godot_print!(
            "Lend {} to {} deadline {}",
            &medium.id,
            &user.account,
            &deadline
        );

        let mut stmt = self.db().prepare(UPDATE_LEND)?;
        stmt.bind(1, user.account.as_str())?;
        stmt.bind(2, deadline.as_str())?;
        stmt.bind(3, medium.id.as_str())?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }

        medium.borrower = user.account.clone();
        medium.deadline = deadline;
        Ok(())
    }

    /// Revokes the borrowing when a borrowed medium is returned.
    fn rental_revoke(&self, medium: &mut Medium) -> api::Result<()> {
        if medium.borrower.is_empty() {
            return Err(api::Error::LogicError);
        }

        let mut stmt = self.db().prepare(UPDATE_REVOKE)?;
        stmt.bind(1, medium.id.as_str())?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        medium.borrower = String::new();
        medium.deadline = String::new();
        Ok(())
    }

    /// Creates a reservation for the borrowed medium.
    fn rental_reserve(&self, medium: &mut Medium, user: &User) -> api::Result<()> {
        if !user.may_borrow {
            return Err(api::Error::RentalUserMayNotBorrow);
        }
        if !medium.borrowable {
            return Err(api::Error::RentalMediumNotBorrowable);
        }
        if !medium.reservation.is_empty() {
            return Err(api::Error::RentalMediumAlreadyReserved);
        }
        if medium.borrower.is_empty() {
            return Err(api::Error::RentalMediumNotBorrowed);
        }
        if medium.borrower == user.account {
            return Err(api::Error::RentalMediumAlreadyBorrowedByUser);
        }

        let mut stmt = self.db().prepare(UPDATE_RESERVE)?;
        stmt.bind(1, user.account.as_str())?;
        stmt.bind(2, medium.id.as_str())?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        medium.reservation = user.account.clone();
        Ok(())
    }

    /// Removes the reservation from the specified medium.
    fn rental_release(&self, medium: &mut Medium) -> api::Result<()> {
        if medium.reservation.is_empty() {
            return Err(api::Error::LogicError);
        }

        let mut stmt = self.db().prepare(UPDATE_RELEASE)?;
        stmt.bind(1, medium.id.as_str())?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        medium.reservation = String::new();
        Ok(())
    }

    /// Return the list of exceeded borrowing periods.
    fn rental_overdues(&self) -> api::Result<DBIter<(Medium, User)>> {
        let stmt = self.db().prepare(QUERY_OVERDUES)?;
        Ok(DBIter::new(stmt))
    }
}

impl ReadStmt for (Medium, User) {
    type Error = api::Error;

    fn read(
        stmt: &sqlite::Statement<'_>,
        columns: &HashMap<String, usize>,
    ) -> api::Result<(Medium, User)> {
        Ok((Medium::read(stmt, columns)?, User::read(stmt, columns)?))
    }
}
