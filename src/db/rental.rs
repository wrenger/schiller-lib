use super::{DBMedium, DBUser};
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

pub trait DatabaseRental {
    fn db(&self) -> &sqlite::Connection;

    /// Lends the medium to the specified user.
    fn rental_lend(&self, medium: &DBMedium, user: &DBUser, days: u32) -> api::Result<DBMedium> {
        if !user.may_borrow {
            return Err(api::Error::RentalUserMayNotBorrow);
        }
        if !medium.borrowable {
            return Err(api::Error::RentalMediumNotBorrowable);
        }
        if !medium.borrower.is_empty() {
            return Err(api::Error::RentalMediumAlreadyBorrowed);
        }
        if !medium.reservation.is_empty() && medium.reservation != user.account {
            return Err(api::Error::RentalMediumAlreadyReserved);
        }

        let deadline = chrono::Utc::today() + chrono::Duration::days(days as _);
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

        let mut medium = medium.clone();
        medium.borrower = user.account.clone();
        medium.deadline = deadline;
        Ok(medium)
    }

    /// Revokes the borrowing when a borrowed medium is returned.
    fn rental_revoke(&self, id: &str) -> api::Result<()> {
        let mut stmt = self.db().prepare(UPDATE_REVOKE)?;
        stmt.bind(1, id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        Ok(())
    }

    /// Creates a reservation for the borrowed medium.
    fn rental_reserve(&self, medium: &DBMedium, user: &DBUser) -> api::Result<()> {
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
        Ok(())
    }

    /// Removes the reservation from the specified medium.
    fn rental_release(&self, id: &str) -> api::Result<()> {
        let mut stmt = self.db().prepare(UPDATE_RELEASE)?;
        stmt.bind(1, id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        Ok(())
    }
}
