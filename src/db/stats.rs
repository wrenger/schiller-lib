use std::collections::HashMap;

use super::{ReadStmt, StatementExt};
use crate::api;

const STATS: &str = r#"
select
(select count(*) from medium) as books,
(select count(distinct name) from author) as authors,
(select count(*) from user) as users,
(select count(*) from medium where borrower <> '') as borrows,
(select count(*) from medium where reservation <> '') as reservations,
(select count(*) from medium where borrower <> '' and JulianDay(date('now')) > JulianDay(date(deadline))) as overdues
"#;

/// Data object for book.
#[derive(Debug, Clone, gdnative::ToVariant, gdnative::FromVariant)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct Stats {
    pub books: usize,
    pub authors: usize,
    pub users: usize,
    pub borrows: usize,
    pub reservations: usize,
    pub overdues: usize,
}

impl ReadStmt for Stats {
    type Error = api::Error;

    fn read(stmt: &sqlite::Statement<'_>, columns: &HashMap<String, usize>) -> api::Result<Stats> {
        Ok(Stats {
            books: stmt.read::<i64>(columns["books"])? as _,
            authors: stmt.read::<i64>(columns["authors"])? as _,
            users: stmt.read::<i64>(columns["users"])? as _,
            borrows: stmt.read::<i64>(columns["borrows"])? as _,
            reservations: stmt.read::<i64>(columns["reservations"])? as _,
            overdues: stmt.read::<i64>(columns["overdues"])? as _,
        })
    }
}

pub trait DatabaseStats {
    fn db(&self) -> &sqlite::Connection;

    fn stats(&self) -> api::Result<Stats> {
        let mut stmt = self.db().prepare(STATS)?;
        if stmt.next()? == sqlite::State::Row {
            ReadStmt::read(&stmt, &stmt.columns())
        } else {
            Err(api::Error::SQLError)
        }
    }
}
