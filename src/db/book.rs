use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use chrono::NaiveDate;
use gluer::metadata;
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

use super::{Categories, Users};
use crate::db::sorted::Sorted;
use crate::error::{Error, Result};
use crate::isbn;

/// Data object for book.
#[metadata]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct Book {
    pub id: String,
    pub isbn: String,
    pub title: String,
    pub publisher: String,
    pub year: i64,
    pub costs: f64,
    #[meta(optional)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub note: String,
    pub borrowable: bool,
    pub category: String,
    pub authors: String,
    #[meta(optional, into = Borrower)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub borrower: Option<Borrower>,
    #[meta(optional, into = String)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reservation: Option<String>,
}

#[metadata]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct Borrower {
    pub user: String,
    #[meta(into = String)]
    pub deadline: NaiveDate,
}

impl Book {
    /// Check if the book is valid
    pub fn validate(&mut self) -> bool {
        self.id = self.id.trim().to_string();
        self.isbn = isbn::parse(&self.isbn).unwrap_or_else(|invalid| invalid);
        self.title = self.title.trim().to_string();
        self.publisher = self.publisher.trim().to_string();
        self.note = self.note.trim().to_string();
        self.category = self.category.trim().to_string();
        self.authors = self.authors.trim().to_string();
        if let Some(reservation) = &mut self.reservation {
            *reservation = reservation.trim().to_string();
        }
        if let Some(borrower) = &mut self.borrower {
            borrower.user = borrower.user.trim().to_string();
        }
        !self.id.is_empty() && !self.title.is_empty()
    }

    /// Fuzzy search score for this book
    pub fn fuzzy(&self, fuzzy: &mut crate::fuzzy::Fuzzy) -> u32 {
        fuzzy.score_many(&[
            (self.id.as_str(), 1), // <- exact match is handled separately
            (self.title.as_str(), 3),
            (self.authors.as_str(), 2),
            (self.isbn.as_str(), 1),
            (self.publisher.as_str(), 1),
            (self.note.as_str(), 1),
            (self.borrower.as_ref().map_or("", |b| b.user.as_str()), 1),
            (self.reservation.as_deref().unwrap_or(""), 1),
        ])
    }
}

/// Book search parameters
#[metadata]
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct BookSearch {
    pub query: String,
    pub category: String,
    pub state: BookState,
    pub offset: usize,
    pub limit: usize,
}

impl Default for BookSearch {
    fn default() -> Self {
        Self {
            query: Default::default(),
            category: Default::default(),
            state: Default::default(),
            offset: 0,
            limit: 100,
        }
    }
}

/// Borrow status of a book
#[metadata]
#[repr(i64)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BookState {
    /// No status
    #[default]
    None = 0,
    /// Can be borrowed
    Borrowable,
    /// Cannot be borrowed
    NotBorrowable,
    /// Is already borrowed
    Borrowed,
    /// Is already reserved
    Reserved,
}

/// Container for all book
#[derive(Serialize, Deserialize, Default)]
pub struct Books {
    #[serde(flatten)]
    pub data: BTreeMap<String, Book>,
}

impl Books {
    /// Return the book with this `id`
    pub fn fetch(&self, id: &str) -> Result<Book> {
        let id = id.trim();
        if !id.is_empty() {
            self.data.get(id).cloned().ok_or(Error::NothingFound)
        } else {
            Err(Error::Arguments)
        }
    }

    /// Add a new book
    pub fn add(&mut self, mut book: Book, categories: &Categories, users: &Users) -> Result<Book> {
        if !book.validate() || !categories.data.contains_key(&book.category) {
            return Err(Error::InvalidBook);
        }
        if let Some(borrower) = &book.borrower
            && !users.data.contains_key(&borrower.user)
        {
            return Err(Error::InvalidBook);
        }
        if let Some(reservation) = &book.reservation
            && !users.data.contains_key(reservation)
        {
            return Err(Error::InvalidBook);
        }

        match self.data.entry(book.id.clone()) {
            Entry::Vacant(v) => {
                v.insert(book.clone());
                Ok(book)
            }
            _ => Err(Error::Duplicate),
        }
    }

    /// Update the book data
    pub fn update(&mut self, id: &str, mut book: Book, categories: &Categories) -> Result<Book> {
        let id = id.trim();
        if id.is_empty() || !book.validate() || !categories.data.contains_key(&book.category) {
            return Err(Error::InvalidBook);
        }

        if id == book.id {
            if let Some(entry) = self.data.get_mut(id) {
                *entry = book.clone();
                return Ok(book);
            }
        } else if self.data.contains_key(id) {
            return match self.data.entry(book.id.clone()) {
                Entry::Vacant(v) => {
                    v.insert(book.clone());
                    self.data.remove(id);
                    Ok(book)
                }
                _ => Err(Error::Duplicate),
            };
        }

        Err(Error::NothingFound)
    }

    /// Delete the corresponding book
    pub fn delete(&mut self, id: &str) -> Result<()> {
        let id = id.trim();
        if !id.is_empty() {
            self.data.remove(id).map(|_| ()).ok_or(Error::NothingFound)
        } else {
            Err(Error::Arguments)
        }
    }

    /// Search specific books
    pub fn search(&self, search: &BookSearch) -> Result<(usize, Vec<Book>)> {
        let mut results = Sorted::<(u32, String, &Book), _>::new(|a, b| {
            a.0.cmp(&b.0)
                .reverse()
                .then_with(|| a.1.cmp(&b.1))
                .then_with(|| a.2.id.cmp(&b.2.id))
        });

        let query = search.query.trim().to_lowercase();
        let mut fuzzy = (!query.is_empty()).then(|| crate::fuzzy::Fuzzy::new(&query));

        // just a very basic keyword search
        for book in self.data.values() {
            // filter by category
            if !search.category.is_empty() && search.category != book.category {
                continue;
            }

            // filter by borrowing state
            match search.state {
                BookState::Borrowable if !book.borrowable => continue,
                BookState::NotBorrowable if book.borrowable => continue,
                BookState::Borrowed if book.borrower.is_none() => continue,
                BookState::Reserved if book.reservation.is_none() => continue,
                _ => {}
            }

            let lower_title = book.title.to_lowercase();

            // Exact match
            let lower_id = book.id.to_ascii_lowercase();
            if query == lower_id {
                results.push((u32::MAX, lower_title, book));
                continue;
            }

            if let Some(fuzzy) = &mut fuzzy {
                let score = book.fuzzy(fuzzy);
                if score > 0 {
                    results.push((score, lower_title, book));
                }
            } else {
                results.push((0, lower_title, book));
            }
        }

        let total = results.len();

        let books = results
            .into_iter()
            .skip(search.offset)
            .take(search.limit)
            .map(|b| b.2.clone())
            .collect();

        Ok((total, books))
    }

    /// Count the number of books in the given category
    pub fn in_category(&self, id: &str) -> Result<usize> {
        let id = id.trim();
        if !id.is_empty() {
            Ok(self.data.values().filter(|b| b.category == id).count())
        } else {
            Err(Error::Arguments)
        }
    }

    /// Generates a new unique id based on the authors surname and the category.
    pub fn generate_id(&self, book: &Book) -> Result<String> {
        let authors = book.authors.trim();
        let prefix = id_prefix(
            authors.split_once(',').map_or(authors, |a| a.0),
            book.category.trim(),
        );
        let id = book.id.trim();
        if id.starts_with(&prefix)
            && id.len() > prefix.len() + 1
            && &id[prefix.len()..=prefix.len()] == " "
        {
            return Ok(id.to_string());
        }

        // query smallest unused id
        let mut max_id = 0usize;
        for key in self.data.keys() {
            if let Some(suffix) = key.strip_prefix(&prefix)
                && let Ok(id) = suffix.trim().parse()
            {
                max_id = max_id.max(id);
            }
        }
        max_id += 1;
        Ok(format!("{prefix} {max_id}"))
    }

    /// Is the user borrowing or reserving by any books
    pub fn is_user_referenced(&self, account: &str) -> bool {
        let account = account.trim();
        if account.is_empty() {
            return false;
        }

        self.data.values().any(|b| {
            matches!(&b.borrower, Some(b) if b.user == account)
                || matches!(&b.reservation, Some(r) if r == account)
        })
    }

    /// Update the account name if it equals `from` to `to`
    pub fn update_user(&mut self, from: &str, to: &str) -> Result<()> {
        let (from, to) = (from.trim(), to.trim());
        if from.is_empty() || to.is_empty() {
            return Err(Error::Arguments);
        }

        for book in self.data.values_mut() {
            if let Some(borrower) = &mut book.borrower
                && borrower.user == from
            {
                borrower.user = to.to_string();
            }
            if let Some(reservation) = &mut book.reservation
                && reservation == from
            {
                *reservation = to.to_string();
            }
        }
        Ok(())
    }

    /// Update the category if it equals `from` to `to`
    pub fn update_category(&mut self, from: &str, to: &str) -> Result<()> {
        let (from, to) = (from.trim(), to.trim());
        if from.is_empty() || to.is_empty() {
            return Err(Error::Arguments);
        }

        for book in self.data.values_mut() {
            if book.category == from {
                book.category = to.to_string();
            }
        }
        Ok(())
    }
}

fn id_prefix(author: &str, category: &str) -> String {
    let mut author = author
        .rsplit_once(' ') // surname
        .map_or(author, |s| s.1)
        .nfd() // decompose -> split ÄÖÜ
        .map(|c| if c == 'ß' { 'S' } else { c })
        .filter(char::is_ascii_alphabetic)
        .map(|c| c.to_ascii_uppercase())
        .take(4)
        .collect::<String>();

    if author.is_empty() {
        author = "XXXX".into();
    }

    let category = if !category.is_empty() {
        category
    } else {
        "XXXX"
    };

    format!("{category} {author}")
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
        let mut db = Database::default();

        assert_eq!(
            db.books
                .search(&BookSearch {
                    query: "".to_owned(),
                    ..BookSearch::default()
                })
                .unwrap()
                .0,
            0
        );

        db.categories
            .add(Category {
                id: "FANT".into(),
                name: "Fantasy".into(),
                section: "General".into(),
            })
            .unwrap();

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
            authors: "John Doe".into(),
            ..Book::default()
        };

        db.books
            .add(book.clone(), &db.categories, &db.users)
            .unwrap();
        let db_book = db
            .books
            .search(&BookSearch {
                query: "".to_owned(),
                ..BookSearch::default()
            })
            .unwrap()
            .1;
        assert_eq!(1, db_book.len());
        assert_eq!(book, db_book[0]);

        // Update book
        db.books
            .update(
                &book.id,
                Book {
                    title: "Another Title".into(),
                    ..book.clone()
                },
                &db.categories,
            )
            .unwrap();

        let db_book = db
            .books
            .search(&BookSearch {
                query: "".to_owned(),
                ..BookSearch::default()
            })
            .unwrap()
            .1;
        assert_eq!(1, db_book.len());
        assert_eq!(db_book[0].title, "Another Title");

        // Remove book
        db.books.delete(&book.id).unwrap();

        assert_eq!(
            db.books
                .search(&BookSearch {
                    query: "".to_owned(),
                    ..BookSearch::default()
                })
                .unwrap()
                .0,
            0
        );
    }
}
