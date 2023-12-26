use std::collections::{btree_map::Entry, BTreeMap};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

use super::Categories;
use crate::db::sorted::Sorted;
use crate::error::{Error, Result};
use crate::isbn;

/// Data object for book.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub authors: String,
    pub borrower: Option<Borrower>,
    pub reservation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct Borrower {
    pub user: String,
    pub deadline: NaiveDate,
}

impl Book {
    fn validate(&mut self) -> bool {
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
}

/// Book search parameters
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

#[repr(i64)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BookState {
    #[default]
    None = 0,
    Borrowable,
    NotBorrowable,
    Borrowed,
    Reserved,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Books {
    #[serde(flatten)]
    pub data: BTreeMap<String, Book>,
}

impl Books {
    pub fn fetch(&self, id: &str) -> Result<Book> {
        self.data.get(id).cloned().ok_or(Error::NothingFound)
    }

    pub fn add(&mut self, mut book: Book, categories: &Categories) -> Result<Book> {
        if !book.validate() || !categories.data.contains_key(&book.category) {
            return Err(Error::InvalidBook);
        }

        match self.data.entry(book.id.clone()) {
            Entry::Vacant(v) => {
                v.insert(book.clone());
                Ok(book)
            }
            _ => Err(Error::InvalidBook),
        }
    }

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
        } else {
            if self.data.remove(id).is_some() {
                return match self.data.entry(book.id.clone()) {
                    Entry::Vacant(v) => {
                        v.insert(book.clone());
                        Ok(book)
                    }
                    _ => Err(Error::InvalidBook),
                };
            }
        }

        Err(Error::NothingFound)
    }

    pub fn delete(&mut self, id: &str) -> Result<()> {
        self.data
            .remove(id.trim())
            .map(|_| ())
            .ok_or(Error::NothingFound)
    }

    /// Search specific books
    pub fn search(&self, search: &BookSearch) -> Result<(usize, Vec<Book>)> {
        fn sort(a: &(usize, String, &Book), b: &(usize, String, &Book)) -> std::cmp::Ordering {
            a.0.cmp(&b.0)
                .reverse()
                .then_with(|| a.1.cmp(&b.1))
                .then_with(|| a.2.id.cmp(&b.2.id))
        }

        let mut results = Sorted::new(sort);

        let keywords = search
            .query
            .split_whitespace()
            .map(str::to_lowercase)
            .collect::<Vec<_>>();

        // just a very basic keyword search
        'books: for book in self.data.values() {
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

            let lower_title = book.title.to_ascii_lowercase();

            if keywords.is_empty() {
                results.push((0, lower_title, &book));
                continue;
            }

            let mut score = 0;
            for keyword in &keywords {
                if lower_title.starts_with(keyword) {
                    score += 3;
                } else if lower_title.contains(keyword) || book.id.to_lowercase().contains(keyword)
                {
                    score += 2;
                } else if book.isbn.to_lowercase().contains(keyword)
                    || book.publisher.to_lowercase().contains(keyword)
                    || book.note.to_lowercase().contains(keyword)
                    || book.authors.to_lowercase().contains(keyword)
                    || matches!(&book.borrower, Some(b) if b.user.to_lowercase().contains(keyword))
                    || matches!(&book.reservation, Some(r) if r.to_lowercase().contains(keyword))
                {
                    score += 1;
                } else {
                    continue 'books;
                }
            }
            if score > 0 {
                results.push((score, lower_title, &book));
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

    pub fn in_category(&self, id: &str) -> Result<usize> {
        let mut count = 0;
        for book in self.data.values() {
            if book.category == id {
                count += 1;
            }
        }
        Ok(count)
    }

    /// Generates a new unique id based on the authors surname and the category.
    pub fn generate_id(&self, book: &Book) -> Result<String> {
        let prefix = id_prefix(
            book.authors.split_once(',').map_or(&book.authors, |a| a.0),
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
        let mut last_id = None;
        for key in self.data.keys() {
            if let Some(suffix) = key.strip_prefix(&prefix) {
                if let Ok(id) = suffix.trim().parse::<usize>() {
                    last_id = Some(id + 1);
                }
            }
        }

        let id = last_id.unwrap_or(1);
        Ok(format!("{prefix} {id}"))
    }

    // Is the user borrowing or reserving by any books
    pub fn is_user_referenced(&self, account: &str) -> bool {
        self.data.values().any(|b| {
            matches!(&b.borrower, Some(b) if b.user == account)
                || matches!(&b.reservation, Some(r) if r == account)
        })
    }

    pub fn update_user(&mut self, from: &str, to: &str) -> Result<()> {
        for book in self.data.values_mut() {
            if let Some(borrower) = &mut book.borrower {
                if borrower.user == from {
                    borrower.user = to.to_string();
                }
            }
            if let Some(reservation) = &mut book.reservation {
                *reservation = to.to_string();
            }
        }
        Ok(())
    }

    pub fn update_category(&mut self, from: &str, to: &str) -> Result<()> {
        for book in self.data.values_mut() {
            if book.category == from {
                book.category = to.to_string();
            }
        }
        Ok(())
    }
}

fn id_prefix(author: &str, category: &str) -> String {
    let mut author_prefix = author
        .rsplit_once(' ') // surname
        .map_or(author, |s| s.1)
        .nfd() // decompose -> split ÄÖÜ
        .map(|c| if c == 'ß' { 'S' } else { c })
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

        db.books.add(book.clone(), &db.categories).unwrap();

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
