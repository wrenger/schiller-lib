use gdnative::prelude::*;

use crate::api::{self, Error};
use crate::db;

/// The Global Project Singleton
#[derive(NativeClass, Debug)]
#[inherit(Node)]
pub struct Project {
    db: Option<db::Database>,
    settings: Option<db::Settings>,
}

#[methods]
impl Project {
    /// Creates a new Project object.
    /// This functions should not be called directly as this class is a singleton.
    fn new(_base: &Node) -> Self {
        info!("sqlite version: {}", rusqlite::version());
        Project {
            db: None,
            settings: None,
        }
    }

    #[inline]
    fn get_db(&self) -> api::Result<&db::Database> {
        self.db.as_ref().ok_or(Error::NoProject)
    }

    /// Returns the program version.
    /// Which may be newer than the project (database) version.
    #[method]
    fn version(&self) -> String {
        crate::PKG_VERSION.into()
    }

    /// Returns info about this project.
    #[method]
    fn about(&self) -> Dictionary {
        let dict = Dictionary::new();
        dict.insert("name", crate::PKG_NAME);
        dict.insert("version", crate::PKG_VERSION);
        dict.insert("repository", crate::PKG_REPOSITORY);
        dict.insert(
            "authors",
            crate::PKG_AUTHORS
                .split(';')
                .map(GodotString::from_str)
                .collect::<PoolArray<_>>(),
        );
        dict.insert("description", crate::PKG_DESCRIPTION);
        dict.insert("license", crate::PKG_LICENSE);
        dict.into_shared()
    }

    /// Opens the specified project and returns if it was upgraded to a new version.
    #[method]
    fn open(&mut self, file: String) -> api::Result<bool> {
        self.close();
        let (db, updated) = db::Database::open(&file)?;
        self.settings = Some(db::settings::fetch(&db)?);
        self.db = Some(db);
        Ok(updated)
    }

    /// Creates a new project database.
    #[method]
    fn create(&mut self, file: String) -> api::Result<()> {
        self.close();
        let db = db::Database::create(&file)?;
        self.settings = Some(db::settings::fetch(&db)?);
        self.db = Some(db);
        Ok(())
    }

    /// Returns the path to the currently opened project or an empty string.
    #[method]
    fn path(&self) -> String {
        if let Some(db) = &self.db {
            db.path().to_str().unwrap_or_default().into()
        } else {
            String::new()
        }
    }

    // Closes the connection to the projects database.
    #[method]
    fn close(&mut self) {
        self.db = None;
        self.settings = None;
    }

    // Book

    /// Returns the book with the given `id`.
    #[method]
    fn book_fetch(&self, id: String) -> api::Result<db::Book> {
        db::book::fetch(self.get_db()?, &id)
    }

    /// Preforms a simple media search with the given `text`.
    #[method]
    fn book_search(&self, text: String) -> api::Result<Vec<db::book::Book>> {
        db::book::search(self.get_db()?, &text)
    }

    /// Performs an advanced media search with the given search parameters.
    #[method]
    fn book_search_advanced(&self, params: db::BookSearch) -> api::Result<Vec<db::book::Book>> {
        db::book::search_advanced(self.get_db()?, &params)
    }

    /// Adds a new book.
    #[method]
    fn book_add(&self, book: db::Book) -> api::Result<()> {
        db::book::add(self.get_db()?, &book)
    }

    /// Updates the book and all references if its id changes.
    #[method]
    fn book_update(&self, previous_id: String, book: db::Book) -> api::Result<()> {
        db::book::update(self.get_db()?, &previous_id, &book)
    }

    /// Deletes the book including the its authors.
    /// Also borrowers & reservations for this book are removed.
    #[method]
    fn book_delete(&self, id: String) -> api::Result<()> {
        db::book::delete(self.get_db()?, &id)
    }

    /// Generates a new book id.
    #[method]
    fn book_generate_id(&self, book: db::Book) -> api::Result<String> {
        db::book::generate_id(self.get_db()?, &book)
    }

    // User

    /// Returns the user with the given `account`.
    #[method]
    fn user_fetch(&self, account: String) -> api::Result<db::User> {
        db::user::fetch(self.get_db()?, &account)
    }

    /// Performs a simple user search with the given `text`.
    #[method]
    fn user_search(&self, text: String) -> api::Result<Vec<db::user::User>> {
        db::user::search(self.get_db()?, &text)
    }

    /// Adds a new user.
    #[method]
    fn user_add(&self, user: db::User) -> api::Result<()> {
        db::user::add(self.get_db()?, &user)
    }

    /// Updates the user and all references if its account changes.
    #[method]
    fn user_update(&self, account: String, user: db::User) -> api::Result<()> {
        db::user::update(self.get_db()?, &account, &user)
    }

    /// Deletes the user.
    /// This includes all its borrows & reservations.
    #[method]
    fn user_delete(&self, account: String) -> api::Result<()> {
        db::user::delete(self.get_db()?, &account)
    }

    /// Deletes the roles from all users and inserts the new roles.
    ///
    /// The roles of all users not contained in the given list are cleared.
    #[method]
    fn user_update_roles(&self, users: Vec<(String, String)>) -> api::Result<()> {
        let db = self.get_db()?;
        let users: Vec<(&str, &str)> = users
            .iter()
            .map(|(u, r)| (u.as_str(), r.as_str()))
            .collect();
        db::user::update_roles(db, &users)
    }

    // Category

    /// Fetches and returns all categories.
    #[method]
    fn category_list(&self) -> api::Result<Vec<db::category::Category>> {
        db::category::list(self.get_db()?)
    }

    /// Adds a new category.
    #[method]
    fn category_add(&self, category: db::Category) -> api::Result<()> {
        db::category::add(self.get_db()?, &category)
    }

    /// Updates the category and all references.
    #[method]
    fn category_update(&self, id: String, category: db::Category) -> api::Result<()> {
        db::category::update(self.get_db()?, &id, &category)
    }

    /// Removes the category or returns a `Error::Logic` if it is still in use.
    #[method]
    fn category_remove(&self, id: String) -> api::Result<()> {
        db::category::delete(self.get_db()?, &id)
    }

    /// Returns the number of books in this category.
    #[method]
    fn category_references(&self, id: String) -> api::Result<i64> {
        db::category::references(self.get_db()?, &id)
    }

    // Lending

    /// Lends the book to the specified user.
    #[method]
    fn lending_lend(&self, mut book: db::Book, user: db::User, days: i64) -> api::Result<db::Book> {
        db::lending::lend(self.get_db()?, &mut book, &user, days)?;
        Ok(book)
    }

    /// Returns the book.
    #[method]
    fn lending_return(&self, mut book: db::Book) -> api::Result<db::Book> {
        db::lending::return_back(self.get_db()?, &mut book)?;
        Ok(book)
    }

    /// Creates a reservation for the borrowed book.
    #[method]
    fn lending_reserve(&self, mut book: db::Book, user: db::User) -> api::Result<db::Book> {
        db::lending::reserve(self.get_db()?, &mut book, &user)?;
        Ok(book)
    }

    /// Removes the reservation from the specified book.
    #[method]
    fn lending_release(&self, mut book: db::Book) -> api::Result<db::Book> {
        db::lending::release(self.get_db()?, &mut book)?;
        Ok(book)
    }

    /// Returns the list of expired borrowing periods.
    #[method]
    fn lending_overdues(&self) -> api::Result<Vec<(db::book::Book, db::user::User)>> {
        db::lending::overdues(self.get_db()?)
    }

    /// Returns the project settings.
    /// They are fetched when opening a project, so that this function only
    /// returns copies of the cached version.
    #[method]
    fn settings_get(&self) -> api::Result<db::Settings> {
        self.settings.clone().ok_or(Error::NoProject)
    }

    /// Updates project settings.
    #[method]
    fn settings_update(&mut self, settings: db::Settings) -> api::Result<()> {
        let db = self.get_db()?;
        db::settings::update(db, &settings)?;
        // Reload cached settings
        self.settings = Some(db::settings::fetch(db)?);
        Ok(())
    }

    /// Returns the project statistics.
    #[method]
    fn stats(&self) -> api::Result<db::Stats> {
        db::stats::fetch(self.get_db()?)
    }
}
