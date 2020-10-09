use std::collections::HashMap;

use crate::api;

use super::raw::{DatabaseExt, StatementExt};
use super::{DBIter, ReadStmt};

const FETCH_MEDIUM: &str = r#"
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

/// Data object for medium.
#[derive(Debug, Clone, gdnative::ToVariant, gdnative::FromVariant)]
pub struct Medium {
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

impl Medium {
    fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.title.is_empty() && !self.category.is_empty()
    }
}

impl ReadStmt for Medium {
    type Error = api::Error;

    fn read(stmt: &sqlite::Statement<'_>, columns: &HashMap<String, usize>) -> api::Result<Medium> {
        Ok(Medium {
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

#[derive(Debug, Clone, Default, gdnative::ToVariant, gdnative::FromVariant)]
pub struct MediumSearch {
    id: String,
    isbn: String,
    title: String,
    publisher: String,
    authors: String,
    year: String,
    category: String,
    note: String,
    user: String,
    state: MediumState,
}

#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediumState {
    None = 0,
    Borrowable,
    NotBorrowable,
    BorrowedOrReserved,
}

impl Default for MediumState {
    fn default() -> MediumState {
        MediumState::None
    }
}

impl From<i64> for MediumState {
    fn from(value: i64) -> Self {
        match value {
            1 => MediumState::Borrowable,
            2 => MediumState::NotBorrowable,
            3 => MediumState::BorrowedOrReserved,
            _ => MediumState::None,
        }
    }
}

impl gdnative::core_types::ToVariant for MediumState {
    fn to_variant(&self) -> gdnative::core_types::Variant {
        (*self as i64).to_variant()
    }
}

impl gdnative::core_types::FromVariant for MediumState {
    fn from_variant(
        variant: &gdnative::core_types::Variant,
    ) -> Result<Self, gdnative::core_types::FromVariantError> {
        i64::from_variant(variant).map(|x| x.into())
    }
}

pub trait DatabaseMedium {
    fn db(&self) -> &sqlite::Connection;

    /// Returns the medium with the given `id`.
    fn medium_fetch(&self, id: &str) -> api::Result<Medium> {
        let mut stmt = self.db().prepare(FETCH_MEDIUM)?;
        stmt.bind(1, id)?;
        if stmt.next()? == sqlite::State::Row {
            Medium::read(&stmt, &stmt.columns())
        } else {
            Err(api::Error::SQLError)
        }
    }

    /// Performes a simple media search with the given `text`.
    fn medium_search(&self, text: &str) -> api::Result<DBIter<Medium>> {
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
    fn medium_search_advanced(&self, params: &MediumSearch) -> api::Result<DBIter<Medium>> {
        gdnative::godot_print!("State: {:?}", params.state);
        let mut stmt = self.db().prepare(QUERY_MEDIA_ADVANCED)?;
        stmt.bind(1, params.id.as_str())?;
        stmt.bind(2, params.isbn.as_str())?;
        stmt.bind(3, params.title.as_str())?;
        stmt.bind(4, params.publisher.as_str())?;
        stmt.bind(5, params.authors.as_str())?;
        if let Some(i) = params.year.find('-') {
            stmt.bind(6, &params.year[..i])?;
            stmt.bind(7, &params.year[i + 1..])?;
        } else if params.year.is_empty() {
            stmt.bind(6, std::i64::MIN)?;
            stmt.bind(7, std::i64::MAX)?;
        } else {
            stmt.bind(6, params.year.as_str())?;
            stmt.bind(7, params.year.as_str())?;
        }
        if !params.category.is_empty() {
            stmt.bind(8, params.category.as_str())?;
        } else {
            stmt.bind(8, "%")?;
        }
        stmt.bind(9, params.note.as_str())?;
        if !params.user.is_empty() {
            stmt.bind(10, params.user.as_str())?;
            stmt.bind(11, params.user.as_str())?;
        } else if params.state == MediumState::BorrowedOrReserved {
            stmt.bind(10, "_%")?;
            stmt.bind(11, "_%")?;
        } else {
            stmt.bind(10, "%")?;
            stmt.bind(11, "%")?;
        }
        match params.state {
            MediumState::Borrowable => stmt.bind(12, 1)?,
            MediumState::NotBorrowable => stmt.bind(12, 0)?,
            _ => stmt.bind(12, "%")?,
        }
        Ok(DBIter::new(stmt))
    }

    /// Adds a new medium.
    fn medium_add(&self, medium: &Medium) -> api::Result<()> {
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
    fn medium_update(&self, previous_id: &str, medium: &Medium) -> api::Result<()> {
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
    fn medium_generate_id(&self, medium: &Medium) -> api::Result<String> {
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
