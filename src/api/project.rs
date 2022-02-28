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
    fn new(_owner: &Node) -> Self {
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
    #[export]
    fn version(&self, _owner: &Node) -> String {
        crate::PKG_VERSION.into()
    }

    /// Returns info about this project.
    #[export]
    fn about(&self, _owner: &Node) -> Dictionary {
        let dict = Dictionary::new();
        dict.insert("name", crate::PKG_NAME);
        dict.insert("version", crate::PKG_VERSION);
        dict.insert("repository", crate::PKG_REPOSITORY);
        dict.insert(
            "authors",
            crate::PKG_AUTHORS
                .split(';')
                .map(GodotString::from_str)
                .collect::<TypedArray<_>>(),
        );
        dict.insert("description", crate::PKG_DESCRIPTION);
        dict.insert("license", crate::PKG_LICENSE);
        dict.into_shared()
    }

    /// Opens the specified project and returns if it was upgraded to a new version.
    #[export]
    fn open(&mut self, owner: &Node, file: String) -> api::Result<bool> {
        self.close(owner);
        let (db, updated) = db::Database::open(&file)?;
        self.settings = Some(db::settings::fetch(&db)?);
        self.db = Some(db);
        Ok(updated)
    }

    /// Creates a new project database.
    #[export]
    fn create(&mut self, owner: &Node, file: String) -> api::Result<()> {
        self.close(owner);
        let db = db::Database::create(&file)?;
        self.settings = Some(db::settings::fetch(&db)?);
        self.db = Some(db);
        Ok(())
    }

    /// Returns the path to the currently opened project or an empty string.
    #[export]
    fn path(&self, _owner: &Node) -> String {
        if let Some(db) = &self.db {
            db.path().to_str().unwrap_or_default().into()
        } else {
            String::new()
        }
    }

    // Closes the connection to the projects database.
    #[export]
    fn close(&mut self, _owner: &Node) {
        self.db = None;
        self.settings = None;
    }

    // Book

    /// Returns the book with the given `id`.
    #[export]
    fn book_fetch(&self, _owner: &Node, id: String) -> api::Result<db::Book> {
        db::book::fetch(self.get_db()?, &id)
    }

    /// Preforms a simple media search with the given `text`.
    #[export]
    fn book_search(&self, _owner: &Node, text: String) -> api::Result<Vec<db::book::Book>> {
        db::book::search(self.get_db()?, &text)
    }

    /// Performs an advanced media search with the given search parameters.
    #[export]
    fn book_search_advanced(
        &self,
        _owner: &Node,
        params: db::BookSearch,
    ) -> api::Result<Vec<db::book::Book>> {
        db::book::search_advanced(self.get_db()?, &params)
    }

    /// Adds a new book.
    #[export]
    fn book_add(&self, _owner: &Node, book: db::Book) -> api::Result<()> {
        db::book::add(self.get_db()?, &book)
    }

    /// Updates the book and all references if its id changes.
    #[export]
    fn book_update(&self, _owner: &Node, previous_id: String, book: db::Book) -> api::Result<()> {
        db::book::update(self.get_db()?, &previous_id, &book)
    }

    /// Deletes the book including the its authors.
    /// Also borrowers & reservations for this book are removed.
    #[export]
    fn book_delete(&self, _owner: &Node, id: String) -> api::Result<()> {
        db::book::delete(self.get_db()?, &id)
    }

    /// Generates a new book id.
    #[export]
    fn book_generate_id(&self, _owner: &Node, book: db::Book) -> api::Result<String> {
        db::book::generate_id(self.get_db()?, &book)
    }

    // User

    /// Returns the user with the given `account`.
    #[export]
    fn user_fetch(&self, _owner: &Node, account: String) -> api::Result<db::User> {
        db::user::fetch(self.get_db()?, &account)
    }

    /// Performs a simple user search with the given `text`.
    #[export]
    fn user_search(&self, _owner: &Node, text: String) -> api::Result<Vec<db::user::User>> {
        db::user::search(self.get_db()?, &text)
    }

    /// Adds a new user.
    #[export]
    fn user_add(&self, _owner: &Node, user: db::User) -> api::Result<()> {
        db::user::add(self.get_db()?, &user)
    }

    /// Updates the user and all references if its account changes.
    #[export]
    fn user_update(&self, _owner: &Node, account: String, user: db::User) -> api::Result<()> {
        db::user::update(self.get_db()?, &account, &user)
    }

    /// Deletes the user.
    /// This includes all its borrows & reservations.
    #[export]
    fn user_delete(&self, _owner: &Node, account: String) -> api::Result<()> {
        db::user::delete(self.get_db()?, &account)
    }

    /// Deletes the roles from all users and inserts the new roles.
    ///
    /// The roles of all users not contained in the given list are cleared.
    #[export]
    fn user_update_roles(&self, _owner: &Node, users: Vec<(String, String)>) -> api::Result<()> {
        let db = self.get_db()?;
        let users: Vec<(&str, &str)> = users
            .iter()
            .map(|(u, r)| (u.as_str(), r.as_str()))
            .collect();
        db::user::update_roles(db, &users)
    }

    // Category

    /// Fetches and returns all categories.
    #[export]
    fn category_list(&self, _owner: &Node) -> api::Result<Vec<db::category::Category>> {
        db::category::list(self.get_db()?)
    }

    /// Adds a new category.
    #[export]
    fn category_add(&self, _owner: &Node, category: db::Category) -> api::Result<()> {
        db::category::add(self.get_db()?, &category)
    }

    /// Updates the category and all references.
    #[export]
    fn category_update(
        &self,
        _owner: &Node,
        id: String,
        category: db::Category,
    ) -> api::Result<()> {
        db::category::update(self.get_db()?, &id, &category)
    }

    /// Removes the category or returns a `Error::Logic` if it is still in use.
    #[export]
    fn category_remove(&self, _owner: &Node, id: String) -> api::Result<()> {
        db::category::delete(self.get_db()?, &id)
    }

    /// Returns the number of books in this category.
    #[export]
    fn category_references(&self, _owner: &Node, id: String) -> api::Result<i64> {
        db::category::references(self.get_db()?, &id)
    }

    // Lending

    /// Lends the book to the specified user.
    #[export]
    fn lending_lend(
        &self,
        _owner: &Node,
        mut book: db::Book,
        user: db::User,
        days: i64,
    ) -> api::Result<db::Book> {
        db::lending::lend(self.get_db()?, &mut book, &user, days)?;
        Ok(book)
    }

    /// Returns the book.
    #[export]
    fn lending_return(&self, _owner: &Node, mut book: db::Book) -> api::Result<db::Book> {
        db::lending::return_back(self.get_db()?, &mut book)?;
        Ok(book)
    }

    /// Creates a reservation for the borrowed book.
    #[export]
    fn lending_reserve(
        &self,
        _owner: &Node,
        mut book: db::Book,
        user: db::User,
    ) -> api::Result<db::Book> {
        db::lending::reserve(self.get_db()?, &mut book, &user)?;
        Ok(book)
    }

    /// Removes the reservation from the specified book.
    #[export]
    fn lending_release(&self, _owner: &Node, mut book: db::Book) -> api::Result<db::Book> {
        db::lending::release(self.get_db()?, &mut book)?;
        Ok(book)
    }

    /// Returns the list of expired borrowing periods.
    #[export]
    fn lending_overdues(&self, _owner: &Node) -> api::Result<Vec<(db::book::Book, db::user::User)>> {
        db::lending::overdues(self.get_db()?)
    }

    /// Returns the project settings.
    /// They are fetched when opening a project, so that this function only
    /// returns copies of the cached version.
    #[export]
    fn settings_get(&self, _owner: &Node) -> api::Result<db::Settings> {
        self.settings.clone().ok_or(Error::NoProject)
    }

    /// Updates project settings.
    #[export]
    fn settings_update(&mut self, _owner: &Node, settings: db::Settings) -> api::Result<()> {
        let db = self.get_db()?;
        db::settings::update(db, &settings)?;
        // Reload cached settings
        self.settings = Some(db::settings::fetch(db)?);
        Ok(())
    }

    /// Returns the project statistics.
    #[export]
    fn stats(&self, _owner: &Node) -> api::Result<db::Stats> {
        db::stats::fetch(self.get_db()?)
    }
}
