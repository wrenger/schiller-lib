use std::collections::HashMap;

use crate::api;

use super::raw::{DatabaseExt, StatementExt};
use super::{DBIter, Database, ReadStmt};

const FETCH: &str = r#"
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

const SEARCH: &str = r#"
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

const SEARCH_ADVANCED: &str = r#"
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

/// Data object for book.
#[derive(Debug, Clone, gdnative::ToVariant, gdnative::FromVariant)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct Book {
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

impl Book {
    fn is_valid(&self) -> bool {
        !self.id.trim().is_empty()
            && !self.title.trim().is_empty()
            && !self.category.trim().is_empty()
    }
}

impl ReadStmt for Book {
    type Error = api::Error;

    fn read(stmt: &sqlite::Statement<'_>, columns: &HashMap<String, usize>) -> api::Result<Book> {
        Ok(Book {
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

/// Book search parameters
#[derive(Debug, Clone, Default, gdnative::ToVariant, gdnative::FromVariant)]
pub struct BookSearch {
    id: String,
    isbn: String,
    title: String,
    publisher: String,
    authors: String,
    year: String,
    category: String,
    note: String,
    user: String,
    state: BookState,
}

#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookState {
    None = 0,
    Borrowable,
    NotBorrowable,
    BorrowedOrReserved,
}

impl Default for BookState {
    fn default() -> BookState {
        BookState::None
    }
}

impl From<i64> for BookState {
    fn from(value: i64) -> Self {
        match value {
            1 => BookState::Borrowable,
            2 => BookState::NotBorrowable,
            3 => BookState::BorrowedOrReserved,
            _ => BookState::None,
        }
    }
}

impl gdnative::core_types::ToVariant for BookState {
    fn to_variant(&self) -> gdnative::core_types::Variant {
        (*self as i64).to_variant()
    }
}

impl gdnative::core_types::FromVariant for BookState {
    fn from_variant(
        variant: &gdnative::core_types::Variant,
    ) -> Result<Self, gdnative::core_types::FromVariantError> {
        i64::from_variant(variant).map(|x| x.into())
    }
}

/// Returns the book with the given `id`.
pub fn fetch(db: &Database, id: &str) -> api::Result<Book> {
    let mut stmt = db.db.prepare(FETCH)?;
    stmt.bind(1, id)?;
    if stmt.next()? == sqlite::State::Row {
        Book::read(&stmt, &stmt.columns())
    } else {
        Err(api::Error::SQLError)
    }
}

/// Performs a simple media search with the given `text`.
pub fn search<'a>(db: &'a Database, text: &str) -> api::Result<DBIter<'a, Book>> {
    let mut stmt = db.db.prepare(SEARCH)?;
    let text = text.trim();
    stmt.bind(1, text)?;
    stmt.bind(2, text)?;
    stmt.bind(3, text)?;
    stmt.bind(4, text)?;
    stmt.bind(5, text)?;
    stmt.bind(6, text)?;
    Ok(DBIter::new(stmt))
}

/// Performs an advanced media search with the given search parameters.
pub fn search_advanced<'a>(db: &'a Database, params: &BookSearch) -> api::Result<DBIter<'a, Book>> {
    gdnative::godot_print!("State: {:?}", params.state);
    let mut stmt = db.db.prepare(SEARCH_ADVANCED)?;
    stmt.bind(1, params.id.trim())?;
    stmt.bind(2, params.isbn.trim())?;
    stmt.bind(3, params.title.trim())?;
    stmt.bind(4, params.publisher.trim())?;
    stmt.bind(5, params.authors.trim())?;
    let year = params.year.trim();
    if let Some(i) = year.find('-') {
        stmt.bind(6, year[..i].trim())?;
        stmt.bind(7, year[i + 1..].trim())?;
    } else if year.is_empty() {
        stmt.bind(6, std::i64::MIN)?;
        stmt.bind(7, std::i64::MAX)?;
    } else {
        stmt.bind(6, year)?;
        stmt.bind(7, year)?;
    }
    let category = params.category.trim();
    if !category.is_empty() {
        stmt.bind(8, category)?;
    } else {
        stmt.bind(8, "%")?;
    }
    stmt.bind(9, params.note.trim())?;
    let user = params.user.trim();
    if !user.is_empty() {
        stmt.bind(10, user)?;
        stmt.bind(11, user)?;
    } else if params.state == BookState::BorrowedOrReserved {
        stmt.bind(10, "_%")?;
        stmt.bind(11, "_%")?;
    } else {
        stmt.bind(10, "%")?;
        stmt.bind(11, "%")?;
    }
    match params.state {
        BookState::Borrowable => stmt.bind(12, 1)?,
        BookState::NotBorrowable => stmt.bind(12, 0)?,
        _ => stmt.bind(12, "%")?,
    }
    Ok(DBIter::new(stmt))
}

/// Adds a new book.
pub fn add(db: &Database, book: &Book) -> api::Result<()> {
    if !book.is_valid() {
        return Err(api::Error::InvalidBook);
    }
    let isbn = if !book.isbn.trim().is_empty() {
        crate::isbn::parse(&book.isbn).unwrap_or_else(|invalid_isbn| invalid_isbn)
    } else {
        String::new()
    };
    let transaction = db.db.transaction()?;
    let mut stmt = db.db.prepare(ADD)?;
    stmt.bind(1, book.id.trim())?;
    stmt.bind(2, isbn.trim())?;
    stmt.bind(3, book.title.trim())?;
    stmt.bind(4, book.publisher.trim())?;
    stmt.bind(5, book.year)?;
    stmt.bind(6, book.costs)?;
    stmt.bind(7, book.note.trim())?;
    stmt.bind(8, book.borrowable as i64)?;
    stmt.bind(9, book.category.trim())?;
    if stmt.next()? != sqlite::State::Done {
        return Err(api::Error::SQLError);
    }
    // Add authors
    for author in &book.authors {
        let mut stmt = db.db.prepare(ADD_AUTHOR)?;
        stmt.bind(1, author.trim())?;
        stmt.bind(2, book.id.trim())?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
    }
    transaction.commit()?;
    Ok(())
}

/// Updates the book and all references if its id changes.
pub fn update(db: &Database, previous_id: &str, book: &Book) -> api::Result<()> {
    let previous_id = previous_id.trim();
    if previous_id.is_empty() || !book.is_valid() {
        return Err(api::Error::InvalidBook);
    }
    let isbn = if !book.isbn.trim().is_empty() {
        crate::isbn::parse(&book.isbn).unwrap_or_else(|invalid_isbn| invalid_isbn)
    } else {
        String::new()
    };
    let transaction = db.db.transaction()?;
    // update book
    let mut stmt = db.db.prepare(UPDATE)?;
    stmt.bind(1, book.id.trim())?;
    stmt.bind(2, isbn.trim())?;
    stmt.bind(3, book.title.trim())?;
    stmt.bind(4, book.publisher.trim())?;
    stmt.bind(5, book.year)?;
    stmt.bind(6, book.costs)?;
    stmt.bind(7, book.note.trim())?;
    stmt.bind(8, book.borrowable as i64)?;
    stmt.bind(9, book.category.trim())?;
    stmt.bind(10, previous_id)?;
    if stmt.next()? != sqlite::State::Done {
        return Err(api::Error::SQLError);
    }

    if previous_id != book.id {
        // update authors
        let mut stmt = db.db.prepare(UPDATE_AUTHORS)?;
        stmt.bind(1, book.id.trim())?;
        stmt.bind(2, previous_id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
    }
    transaction.commit()?;
    Ok(())
}

/// Deletes the book including the its authors.
/// Also borrowers & reservations for this book are removed.
pub fn delete(db: &Database, id: &str) -> api::Result<()> {
    let id = id.trim();
    if id.is_empty() {
        return Err(api::Error::InvalidBook);
    }

    let transaction = db.db.transaction()?;
    let mut stmt = db.db.prepare(DELETE)?;
    stmt.bind(1, id)?;
    if stmt.next()? != sqlite::State::Done {
        return Err(api::Error::SQLError);
    }

    // delete missing authors
    db.db.execute(DELETE_UNUSED_AUTHORS)?;
    transaction.commit()?;
    Ok(())
}

/// Generates a new unique id based on the authors surname and the category.
pub fn generate_id(db: &Database, book: &Book) -> api::Result<String> {
    let prefix = id_prefix(
        book.authors.first().map(|s| s.trim()).unwrap_or_default(),
        book.category.trim(),
    );
    println!("Prefix {}", prefix);
    let id = book.id.trim();
    if id.starts_with(&prefix)
        && id.len() > prefix.len() + 1
        && &id[prefix.len()..prefix.len() + 1] == " "
    {
        return Ok(id.to_string());
    }

    let mut stmt = db.db.prepare(UNUSED_ID)?;
    stmt.bind(1, prefix.len() as i64)?;
    stmt.bind(2, prefix.as_str())?;
    if stmt.next()? != sqlite::State::Row {
        return Err(api::Error::SQLError);
    }
    let id = stmt.read::<i64>(0)? + 1;
    Ok(format!("{} {}", prefix, id))
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
    use super::super::*;
    use super::*;

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

    #[test]
    fn add_update_remove_book() {
        let db = Database::memory().unwrap();
        structure::create(&db, PKG_VERSION).unwrap();

        assert_eq!(book::search(&db, "").unwrap().count(), 0);

        // New book
        let book = Book {
            id: "FANT DOE 1".into(),
            isbn: "".into(),
            title: "Demo Test Book".into(),
            publisher: "Test".into(),
            year: 2020,
            costs: 7.5,
            note: "Not a real book".into(),
            borrowable: true,
            category: "FANT".into(),
            authors: vec!["John Doe".into()],
            ..Book::default()
        };

        book::add(&db, &book).unwrap();

        let db_book = book::search(&db, "").unwrap().next().unwrap();
        assert_eq!(book, db_book);

        // Update book
        book::update(
            &db,
            &book.id,
            &Book {
                title: "Another Title".into(),
                ..book.clone()
            },
        )
        .unwrap();

        let db_book = book::search(&db, "").unwrap().next().unwrap();
        assert_eq!(db_book.title, "Another Title");

        // Remove book
        book::delete(&db, &book.id).unwrap();

        assert_eq!(book::search(&db, "").unwrap().count(), 0);
    }
}
