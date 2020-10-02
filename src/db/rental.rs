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
    fn rental_lend(&self, id: &str, account: &str, deadline: &str) -> api::Result<()> {
        let mut stmt = self.db().prepare(UPDATE_LEND)?;
        stmt.bind(1, account)?;
        stmt.bind(2, deadline)?;
        stmt.bind(3, id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        Ok(())
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
    fn rental_reserve(&self, id: &str, account: &str) -> api::Result<()> {
        let mut stmt = self.db().prepare(UPDATE_RESERVE)?;
        stmt.bind(1, account)?;
        stmt.bind(2, id)?;
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
