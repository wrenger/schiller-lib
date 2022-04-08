use gdnative::core_types::{FromVariant, FromVariantError};
use unicode_normalization::UnicodeNormalization;

use crate::api;

use super::{DBIter, Database, FromRow};

use gdnative::derive::{FromVariant, ToVariant};

const FETCH: &str = "\
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
    reservation \
    \
    from medium \
    left join author on author.medium=id \
    where id=? \
    group by id \
";

const SEARCH: &str = "\
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
    reservation \
    \
    from medium \
    left join author on author.medium=id \
    group by id \
    having id like '%'||?1||'%' \
    or isbn like '%'||?1||'%' \
    or title like '%'||?1||'%' \
    or publisher like '%'||?1||'%' \
    or note like '%'||?1||'%' \
    or authors like '%'||?1||'%' \
";

const SEARCH_ADVANCED: &str = "\
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
    reservation \
    \
    from medium \
    left join author on author.medium=id \
    group by id \
    having id like '%'||?||'%' \
    and isbn like '%'||?||'%' \
    and title like '%'||?||'%' \
    and publisher like '%'||?||'%' \
    and authors like '%'||?||'%' \
    and year between ? and ? \
    and category like ? \
    and note like '%'||?||'%' \
    and (borrower like '%'||?||'%' or reservation like '%'||?||'%') \
    and borrowable like ? \
";

const ADD: &str = "\
    insert into medium values (?, ?, ?, ?, ?, ?, ?, ?, ?, '', '', '') \
";

const ADD_AUTHOR: &str = "\
    insert or ignore into author values (?, ?) \
";
const UPDATE: &str = "\
    update medium \
    set id=?, isbn=?, title=?, publisher=?, year=?, costs=?, note=?, borrowable=?, category=? \
    where id=? \
";
const UPDATE_AUTHORS: &str = "\
    update author set medium=? where medium=? \
";
const DELETE: &str = "\
    delete from medium where id=? \
";
const DELETE_UNUSED_AUTHORS: &str = "\
    delete from author where medium not in (select id from medium) \
";
// CAST returns 0 on failure, MAX returns NULL if empty
const UNUSED_ID: &str = "\
    select ifnull(max(cast(substr(id, length(?1) + 1) as integer)), 0) \
    from medium where id like ?1||'%' \
    order by id \
";

/// Data object for book.
#[derive(Debug, Clone, ToVariant, FromVariant)]
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
        !self.id.trim().is_empty() && !self.title.trim().is_empty()
    }
}

impl FromRow for Book {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Book> {
        Ok(Book {
            id: row.get("id")?,
            isbn: row.get("isbn")?,
            title: row.get("title")?,
            publisher: row.get("publisher")?,
            year: row.get("year")?,
            costs: row.get("costs")?,
            note: row.get("note")?,
            borrowable: row.get("borrowable")?,
            category: row.get("category")?,
            authors: row
                .get::<&str, String>("authors")?
                .split(',')
                .map(|a| a.to_string())
                .collect(),
            borrower: row.get("borrower")?,
            deadline: row.get("deadline")?,
            reservation: row.get("reservation")?,
        })
    }
}

/// Book search parameters
#[derive(Debug, Clone, Default, FromVariant)]
pub struct BookSearch {
    id: String,
    isbn: String,
    title: String,
    publisher: String,
    authors: String,
    year: YearRange,
    category: String,
    note: String,
    user: String,
    state: BookState,
}

#[derive(Debug, Clone)]
struct YearRange(u16, u16);

impl Default for YearRange {
    fn default() -> Self {
        Self(0, u16::MAX)
    }
}

impl FromVariant for YearRange {
    fn from_variant(variant: &gdnative::core_types::Variant) -> Result<Self, FromVariantError> {
        let str = String::from_variant(variant)?;
        let str = str.trim();
        if str.is_empty() {
            Ok(Self::default())
        } else if let Some((start, end)) = str.split_once('-') {
            let start = start.trim().parse().unwrap_or_default();
            let end = end.trim().parse().unwrap_or(u16::MAX);
            Ok(Self(start, end))
        } else {
            let year = str.trim().parse().unwrap_or_default();
            Ok(Self(year, year))
        }
    }
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
    fn from_variant(variant: &gdnative::core_types::Variant) -> Result<Self, FromVariantError> {
        i64::from_variant(variant).map(|x| x.into())
    }
}

/// Returns the book with the given `id`.
pub fn fetch(db: &Database, id: &str) -> api::Result<Book> {
    Ok(db.con.query_row(FETCH, [id], Book::from_row)?)
}

/// Performs a simple media search with the given `text`.
pub fn search(db: &Database, text: &str) -> api::Result<Vec<Book>> {
    let mut stmt = db.con.prepare(SEARCH)?;
    let rows = stmt.query(rusqlite::params![text.trim()])?;
    DBIter::new(rows).collect()
}

/// Performs an advanced media search with the given search parameters.
pub fn search_advanced(db: &Database, params: &BookSearch) -> api::Result<Vec<Book>> {
    let mut stmt = db.con.prepare(SEARCH_ADVANCED)?;
    let user = params.user.trim();
    let user = if !user.is_empty() {
        user
    } else if params.state == BookState::BorrowedOrReserved {
        "_%"
    } else {
        "%"
    };
    let rows = stmt.query(rusqlite::params![
        params.id.trim(),
        params.isbn.trim(),
        params.title.trim(),
        params.publisher.trim(),
        params.authors.trim(),
        params.year.0,
        params.year.1,
        params.category.trim(),
        params.note.trim(),
        user,
        user,
        match params.state {
            BookState::Borrowable => "1",
            BookState::NotBorrowable => "0",
            _ => "%",
        }
    ])?;
    DBIter::new(rows).collect()
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
    let transaction = db.transaction()?;
    transaction.execute(
        ADD,
        rusqlite::params![
            book.id.trim(),
            isbn.trim(),
            book.title.trim(),
            book.publisher.trim(),
            book.year,
            book.costs,
            book.note.trim(),
            book.borrowable,
            book.category.trim(),
        ],
    )?;

    // Add authors
    for author in &book.authors {
        transaction.execute(ADD_AUTHOR, [author.trim(), book.id.trim()])?;
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
    let transaction = db.transaction()?;
    // update book
    transaction.execute(
        UPDATE,
        rusqlite::params![
            book.id.trim(),
            isbn.trim(),
            book.title.trim(),
            book.publisher.trim(),
            book.year,
            book.costs,
            book.note.trim(),
            book.borrowable,
            book.category.trim(),
            previous_id
        ],
    )?;

    if previous_id != book.id {
        // update authors
        transaction.execute(UPDATE_AUTHORS, [book.id.trim(), previous_id])?;
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

    let transaction = db.transaction()?;
    transaction.execute(DELETE, [id])?;

    // delete missing authors
    transaction.execute(DELETE_UNUSED_AUTHORS, [])?;
    transaction.commit()?;
    Ok(())
}

/// Generates a new unique id based on the authors surname and the category.
pub fn generate_id(db: &Database, book: &Book) -> api::Result<String> {
    let prefix = id_prefix(
        book.authors.first().map(|s| s.trim()).unwrap_or_default(),
        book.category.trim(),
    );
    let id = book.id.trim();
    if id.starts_with(&prefix)
        && id.len() > prefix.len() + 1
        && &id[prefix.len()..prefix.len() + 1] == " "
    {
        return Ok(id.to_string());
    }

    let id = db
        .con
        .query_row(UNUSED_ID, rusqlite::params![prefix.as_str()], |v| {
            v.get::<usize, usize>(0).map(|v| v + 1)
        })?;
    Ok(format!("{prefix} {id}"))
}

fn id_prefix(author: &str, category: &str) -> String {
    let mut author_prefix = author
        .rsplit_once(' ') // surname
        .map(|s| s.1)
        .unwrap_or(author)
        .nfd() // decompose -> split ÄÖÜ
        .map(|c| (c == 'ß').then(|| 'S').unwrap_or(c))
        .filter(char::is_ascii_alphabetic)
        .map(|c| c.to_ascii_uppercase())
        .collect::<String>();

    if author_prefix.is_empty() {
        author_prefix = "XXXX".into();
    }

    let category = if !category.is_empty() {
        category
    } else {
        "XXXX"
    };

    format!(
        "{category} {}",
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
        assert_eq!(id_prefix("äÖüß", "FANT"), "FANT AOUS".to_string());
        assert_eq!(
            id_prefix("Remigius Bäumer", "RErk"),
            "RErk BAUM".to_string()
        );
        assert_eq!(id_prefix("Isabel Abedi", ""), "XXXX ABED".to_string());
    }

    #[test]
    fn add_update_remove_book() {
        let db = Database::memory().unwrap();
        structure::create(&db, PKG_VERSION).unwrap();

        assert_eq!(book::search(&db, "").unwrap().len(), 0);

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

        let db_book = &book::search(&db, "").unwrap()[0];
        assert_eq!(&book, db_book);

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

        let db_book = &book::search(&db, "").unwrap()[0];
        assert_eq!(db_book.title, "Another Title");

        // Remove book
        book::delete(&db, &book.id).unwrap();

        assert_eq!(book::search(&db, "").unwrap().len(), 0);
    }
}
