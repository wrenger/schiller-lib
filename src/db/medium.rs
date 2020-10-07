use std::collections::HashMap;

use crate::api;

use super::raw::{DatabaseExt, StatementExt};
use super::{DBIter, ReadStmt};

const GET_MEDIUM: &str = r#"
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
reservation
from medium
left join author on author.medium=id
where id=?
group by id
"#;

const QUERY_MEDIA: &str = r#"
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
reservation
from medium
left join author on author.medium=id
group by id
having id like '%'||?||'%'
or isbn like '%'||?||'%'
or title like '%'||?||'%'
or publisher like '%'||?||'%'
or note like '%'||?||'%'
or authors like '%'||?||'%'
"#;

const QUERY_MEDIA_ADVANCED: &str = r#"
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
reservation
from medium
left join author on author.medium=id
group by id
having id like '%'||?||'%'
and isbn like '%'||?||'%'
and title like '%'||?||'%'
and publisher like '%'||?||'%'
and authors like '%'||?||'%'
and year between ? and ?
and category like ?
and note like '%'||?||'%'
and (borrower like '%'||?||'%' or reservation like '%'||?||'%')
and borrowable like ?
"#;

const ADD: &str = r#"
insert into medium values (?, ?, ?, ?, ?, ?, ?, ?, ?, '', '', '')
"#;

const ADD_AUTHOR: &str = r#"
insert or ignore into author values (?, ?)
"#;
const UPDATE: &str = r#"
update medium set id=?, isbn=?, title=?, publisher=?, year=?, costs=?, note=?, borrowable=?, category=? where id=?
"#;
const UPDATE_AUTHORS: &str = r#"
update author set medium=? where medium=?
"#;

const DELETE: &str = r#"
delete from medium where id=?
"#;
const DELETE_UNUSED_AUTHORS: &str = r#"
delete from author where medium not in (select id from medium)
"#;

const UNUSED_ID: &str = r#"
select max(substr(id, ? + 2)) from medium where id like ?||'%' order by id
"#;

#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DBMediumState {
    None = 0,
    Borrowable,
    NotBorrowable,
    BorrowedOrReserved,
}

impl From<i64> for DBMediumState {
    fn from(value: i64) -> Self {
        match value {
            1 => DBMediumState::Borrowable,
            2 => DBMediumState::NotBorrowable,
            3 => DBMediumState::BorrowedOrReserved,
            _ => DBMediumState::None,
        }
    }
}

/// Data object for medium.
#[derive(Debug, Clone)]
pub struct DBMedium {
    pub id: String,
    pub isbn: String,
    pub title: String,
    pub publisher: String,
    pub year: i64,
    pub costs: f64,
    pub note: String,
    pub borrowable: bool,
    pub category: String,
    pub authors: Vec<String>,
    pub borrower: String,
    pub deadline: String,
    pub reservation: String,
}

impl DBMedium {
    fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.title.is_empty() && !self.category.is_empty()
    }
}

impl ReadStmt for DBMedium {
    type Error = api::Error;

    fn read(
        stmt: &sqlite::Statement<'_>,
        columns: &HashMap<String, usize>,
    ) -> api::Result<DBMedium> {
        Ok(DBMedium {
            id: stmt.read(columns["id"])?,
            isbn: stmt.read(columns["isbn"])?,
            title: stmt.read(columns["title"])?,
            publisher: stmt.read(columns["publisher"])?,
            year: stmt.read(columns["year"])?,
            costs: stmt.read(columns["costs"])?,
            note: stmt.read(columns["note"])?,
            borrowable: stmt.read::<i64>(columns["borrowable"])? != 0,
            category: stmt.read(columns["category"])?,
            authors: stmt
                .read::<String>(columns["authors"])?
                .split(',')
                .map(|a| a.to_string())
                .collect(),
            borrower: stmt.read(columns["borrower"])?,
            deadline: stmt.read(columns["deadline"])?,
            reservation: stmt.read(columns["reservation"])?,
        })
    }
}

pub trait DatabaseMedium {
    fn db(&self) -> &sqlite::Connection;

    /// Returns the medium with the given `id`.
    fn medium_get(&self, id: &str) -> api::Result<DBMedium> {
        let mut stmt = self.db().prepare(GET_MEDIUM)?;
        stmt.bind(1, id)?;
        if stmt.next()? == sqlite::State::Row {
            DBMedium::read(&stmt, &stmt.columns())
        } else {
            Err(api::Error::SQLError)
        }
    }

    /// Performes a simple media search with the given `text`.
    fn medium_search(&self, text: &str) -> api::Result<DBIter<DBMedium>> {
        let mut stmt = self.db().prepare(QUERY_MEDIA)?;
        stmt.bind(1, text)?;
        stmt.bind(2, text)?;
        stmt.bind(3, text)?;
        stmt.bind(4, text)?;
        stmt.bind(5, text)?;
        stmt.bind(6, text)?;
        Ok(DBIter::new(stmt))
    }

    /// Performes a simple media search with the given `text`.
    fn medium_search_advanced(
        &self,
        id: &str,
        isbn: &str,
        title: &str,
        publisher: &str,
        authors: &str,
        year: &str,
        category: &str,
        note: &str,
        user: &str,
        state: DBMediumState,
    ) -> api::Result<DBIter<DBMedium>> {
        gdnative::godot_print!("State: {:?}", state);
        let mut stmt = self.db().prepare(QUERY_MEDIA_ADVANCED)?;
        stmt.bind(1, id)?;
        stmt.bind(2, isbn)?;
        stmt.bind(3, title)?;
        stmt.bind(4, publisher)?;
        stmt.bind(5, authors)?;
        if let Some(i) = year.find('-') {
            stmt.bind(6, &year[..i])?;
            stmt.bind(7, &year[i + 1..])?;
        } else if year.is_empty() {
            stmt.bind(6, std::i64::MIN)?;
            stmt.bind(7, std::i64::MAX)?;
        } else {
            stmt.bind(6, year)?;
            stmt.bind(7, year)?;
        }
        if !category.is_empty() {
            stmt.bind(8, category)?;
        } else {
            stmt.bind(8, "%")?;
        }
        stmt.bind(9, note)?;
        if !user.is_empty() {
            stmt.bind(10, user)?;
            stmt.bind(11, user)?;
        } else if state == DBMediumState::BorrowedOrReserved {
            stmt.bind(10, "_%")?;
            stmt.bind(11, "_%")?;
        } else {
            stmt.bind(10, "%")?;
            stmt.bind(11, "%")?;
        }
        match state {
            DBMediumState::Borrowable => stmt.bind(12, 1)?,
            DBMediumState::NotBorrowable => stmt.bind(12, 0)?,
            _ => stmt.bind(12, "%")?,
        }
        Ok(DBIter::new(stmt))
    }

    /// Adds a new medium.
    fn medium_add(&self, medium: &DBMedium) -> api::Result<()> {
        if !medium.is_valid() {
            return Err(api::Error::LogicError);
        }
        // Add medium
        let transaction = self.db().transaction()?;
        let mut stmt = self.db().prepare(ADD)?;
        stmt.bind(1, medium.id.as_str())?;
        stmt.bind(2, medium.isbn.as_str())?;
        stmt.bind(3, medium.title.as_str())?;
        stmt.bind(4, medium.publisher.as_str())?;
        stmt.bind(5, medium.year)?;
        stmt.bind(6, medium.costs)?;
        stmt.bind(7, medium.note.as_str())?;
        stmt.bind(8, medium.borrowable as i64)?;
        stmt.bind(9, medium.category.as_str())?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        // Add authors
        for author in &medium.authors {
            let mut stmt = self.db().prepare(ADD_AUTHOR)?;
            stmt.bind(1, author.as_str())?;
            stmt.bind(2, medium.id.as_str())?;
            if stmt.next()? != sqlite::State::Done {
                return Err(api::Error::SQLError);
            }
        }
        transaction.commit()?;
        Ok(())
    }

    /// Updates the medium and all references if its id changes.
    fn medium_update(&self, previous_id: &str, medium: &DBMedium) -> api::Result<()> {
        if !medium.is_valid() {
            return Err(api::Error::LogicError);
        }
        let transaction = self.db().transaction()?;
        // update medium
        let mut stmt = self.db().prepare(UPDATE)?;
        stmt.bind(1, medium.id.as_str())?;
        stmt.bind(2, medium.isbn.as_str())?;
        stmt.bind(3, medium.title.as_str())?;
        stmt.bind(4, medium.publisher.as_str())?;
        stmt.bind(5, medium.year)?;
        stmt.bind(6, medium.costs)?;
        stmt.bind(7, medium.note.as_str())?;
        stmt.bind(8, medium.borrowable as i64)?;
        stmt.bind(9, medium.category.as_str())?;
        stmt.bind(10, previous_id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }

        if previous_id != medium.id {
            // update authors
            let mut stmt = self.db().prepare(UPDATE_AUTHORS)?;
            stmt.bind(1, medium.id.as_str())?;
            stmt.bind(2, previous_id)?;
            if stmt.next()? != sqlite::State::Done {
                return Err(api::Error::SQLError);
            }
        }
        transaction.commit()?;
        Ok(())
    }

    /// Deletes the medium including the its authors.
    /// Also borrowers & reservations for this medium are removed.
    fn medium_delete(&self, id: &str) -> api::Result<()> {
        let transaction = self.db().transaction()?;
        // delete medium
        let mut stmt = self.db().prepare(DELETE)?;
        stmt.bind(1, id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }

        // delete missing authors
        self.db().execute(DELETE_UNUSED_AUTHORS)?;
        transaction.commit()?;
        Ok(())
    }

    /// Generates a new unique id based on the authors surname and the category.
    fn medium_generate_id(&self, medium: &DBMedium) -> api::Result<String> {
        let prefix = id_prefix(
            medium
                .authors
                .first()
                .map(|s| s.as_str())
                .unwrap_or_default(),
            &medium.category,
        );
        println!("Prefix {}", prefix);
        if medium.id.starts_with(&prefix)
            && medium.id.len() > prefix.len() + 1
            && &medium.id[prefix.len()..prefix.len() + 1] == " "
        {
            return Ok(medium.id.clone());
        }

        let mut stmt = self.db().prepare(UNUSED_ID)?;
        stmt.bind(1, prefix.len() as i64)?;
        stmt.bind(2, prefix.as_str())?;
        if stmt.next()? != sqlite::State::Row {
            return Err(api::Error::SQLError);
        }
        let id = stmt.read::<i64>(0)? + 1;
        Ok(format!("{} {}", prefix, id))
    }
}

fn id_prefix(author: &str, category: &str) -> String {
    let mut author_prefix = author[author.rfind(' ').map(|i| i + 1).unwrap_or_default()..]
        .replace(&['ä', 'Ä'][..], "A")
        .replace(&['ö', 'Ö'][..], "O")
        .replace(&['ü', 'Ü'][..], "U")
        .replace('ß', "S")
        .replace(|x: char| !x.is_ascii_alphabetic(), "")
        .to_ascii_uppercase();
    if author_prefix.is_empty() {
        author_prefix = "XXXX".into();
    }
    format!(
        "{} {}",
        category,
        &author_prefix[..author_prefix.len().min(4)],
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn id_prefix() {
        use super::id_prefix;
        assert_eq!(id_prefix("Isabel Abedi", "FANT"), "FANT ABED".to_string());
        assert_eq!(id_prefix("Isabel Äbedi", "FANT"), "FANT ABED".to_string());
        assert_eq!(id_prefix("", "FANT"), "FANT XXXX".to_string());
        assert_eq!(id_prefix("äÖü", "FANT"), "FANT AOU".to_string());
        assert_eq!(
            id_prefix("Remigius Bäumer", "RErk"),
            "RErk BAUM".to_string()
        );
    }
}
