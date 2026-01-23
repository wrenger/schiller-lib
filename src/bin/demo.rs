use std::path::Path;

use schiller_lib::db::{AtomicDatabase, Book, Category};
use schiller_lib::error::Error;
use schiller_lib::{provider, util};
use tracing::error;

/// Create a demo database with a few books
#[tokio::main]
async fn main() {
    util::logging();
    let client = reqwest::Client::new();

    // About 2000 are realistic for a school library
    let book_count = 2000;
    let mut books = Vec::new();
    for p in 0..(book_count / 100) {
        let b = provider::dnb::query(&client, "jhr=2020 and spr=ger and mat=books", p)
            .await
            .unwrap();
        let len = b.len();
        println!("{len} books");
        books.extend(b);
        if len < 100 {
            break;
        }
    }
    println!("=> {} books", books.len());

    let db = AtomicDatabase::create(Path::new("test/demo.json")).unwrap();
    let mut db = db.write();
    db.categories
        .add(Category {
            id: "demo".into(),
            name: "Demo".into(),
            section: "Demo".into(),
        })
        .unwrap();
    db.categories
        .add(Category {
            id: "long".into(),
            name: "Long Title".into(),
            section: "Demo".into(),
        })
        .unwrap();
    for mut record in books {
        let mut book = Book {
            id: String::new(),
            isbn: record.isbns.pop().unwrap_or_default(),
            category: if record.data.title.len() > 50 {
                "long".into()
            } else {
                "demo".into()
            },
            title: record.data.title,
            publisher: record.data.publisher,
            year: 2020,
            costs: record.data.costs,
            note: String::new(),
            borrowable: true,
            authors: record.data.authors.join(", "),
            borrower: None,
            reservation: None,
        };
        let db = &mut *db;
        let id = db.books.generate_id(&book).unwrap();
        book.id = id;
        if book.validate() {
            // ignore duplicates
            match db.books.add(book, &db.categories, &db.users) {
                Ok(_) => {}
                Err(Error::Duplicate) => {}
                Err(e) => error!("Failed to add book: {e:?}"),
            }
        } else {
            error!("Invalid book {}", book.title);
        }
    }

    let users = provider::user::load_all(Path::new("test/csv/users.csv"), b',').unwrap();
    for user in users {
        db.users.add(user).unwrap();
    }
}
