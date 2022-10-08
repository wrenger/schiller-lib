use super::{Database, FromRow};
use crate::api;

use gdnative::derive::{FromVariant, ToVariant};

/// Data object for book.
#[derive(Debug, Clone, ToVariant, FromVariant)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct Stats {
    pub books: usize,
    pub authors: usize,
    pub users: usize,
    pub borrows: usize,
    pub reservations: usize,
    pub overdues: usize,
}

impl FromRow for Stats {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Stats> {
        Ok(Stats {
            books: row.get("books")?,
            authors: row.get("authors")?,
            users: row.get("users")?,
            borrows: row.get("borrows")?,
            reservations: row.get("reservations")?,
            overdues: row.get("overdues")?,
        })
    }
}

pub fn fetch(db: &Database) -> api::Result<Stats> {
    const STATS: &str = "\
        select \
        (select count(*) from medium) as books, \
        (select count(distinct name) from author) as authors, \
        (select count(*) from user) as users, \
        (select count(*) from medium where borrower <> '') as borrows, \
        (select count(*) from medium where reservation <> '') as reservations, \
        (select count(*) from medium where borrower <> '' and JulianDay(date('now')) > JulianDay(date(deadline))) as  overdues \
    ";
    Ok(db.con.query_row(STATS, [], Stats::from_row)?)
}
