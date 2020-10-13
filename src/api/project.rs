use std::iter::FromIterator;

use gdnative::prelude::*;

use super::debug;
use crate::api::{self, Error};
use crate::db::{
    self, Database, DatabaseBook, DatabaseCategory, DatabaseLending, DatabaseSettings, DatabaseUser,
};

/// The Global Project Singleton
#[derive(NativeClass, Debug)]
#[inherit(Node)]
pub struct Project {
    db: Option<Database>,
    settings: Option<db::Settings>,
}

#[methods]
impl Project {
    /// Creates a new Project object.
    /// This functions should not be called directly as this class is a singleton.
    fn new(_owner: &Node) -> Self {
        godot_print!("sqlite version: {}", sqlite::version());
        Project {
            db: None,
            settings: None,
        }
    }

    /// Returns the program version.
    /// Which may be newer than the project (database) version.
    #[export]
    fn version(&self, _owner: &Node) -> String {
        db::PKG_VERSION.into()
    }

    /// Opens the specified project and returns if it was successful.
    #[export]
    fn open(&mut self, owner: &Node, file: GodotString) -> api::Result<bool> {
        let _timer = debug::timer();
        self.close(owner);
        let (db, updated) = Database::open(&file.to_string())?;
        self.settings = Some(db.settings_fetch()?);
        self.db = Some(db);
        Ok(updated)
    }

    /// Creates a new project database.
    #[export]
    fn create(&mut self, owner: &Node, file: GodotString) -> api::Result<()> {
        let _timer = debug::timer();
        self.close(owner);
        let db = Database::create(&file.to_string())?;
        self.settings = Some(db.settings_fetch()?);
        self.db = Some(db);
        Ok(())
    }

    /// Returns the path to the currently opened project or an empty string.
    #[export]
    fn path(&self, _owner: &Node) -> GodotString {
        if let Some(db) = &self.db {
            db.path().to_str().unwrap_or_default().into()
        } else {
            GodotString::new()
        }
    }

    // Closes the connection to the projects database.
    #[export]
    fn close(&mut self, _owner: &Node) {
        self.db = None;
        self.settings = None;
    }

    // Book

    /// Preforms a simple media search with the given `text`.
    #[export]
    fn book_search(&self, _owner: &Node, text: GodotString) -> api::Result<VariantArray> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.book_search(&text.to_string())?;
        Ok(VariantArray::from_iter(result).into_shared())
    }

    /// Performs an advanced media search with the given search parameters.
    #[export]
    fn book_search_advanced(
        &self,
        _owner: &Node,
        params: db::BookSearch,
    ) -> api::Result<VariantArray> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.book_search_advanced(&params)?;
        Ok(VariantArray::from_iter(result).into_shared())
    }

    /// Adds a new book.
    #[export]
    fn book_add(&self, _owner: &Node, book: db::Book) -> api::Result<()> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.book_add(&book)
    }

    /// Updates the book and all references if its id changes.
    #[export]
    fn book_update(&self, _owner: &Node, previous_id: String, book: db::Book) -> api::Result<()> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.book_update(&previous_id, &book)
    }

    /// Deletes the book including the its authors.
    /// Also borrowers & reservations for this book are removed.
    #[export]
    fn book_delete(&self, _owner: &Node, id: String) -> api::Result<()> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.book_delete(&id)
    }

    /// Generates a new book id.
    #[export]
    fn book_generate_id(&self, _owner: &Node, book: db::Book) -> api::Result<String> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.book_generate_id(&book)
    }

    // User

    /// Returns the user with the given `account`.
    #[export]
    fn user_fetch(&self, _owner: &Node, account: GodotString) -> api::Result<db::User> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.user_fetch(&account.to_string())
    }

    /// Performs a simple user search with the given `text`.
    #[export]
    fn user_search(&self, _owner: &Node, text: GodotString) -> api::Result<VariantArray> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.user_search(&text.to_string())?;
        Ok(VariantArray::from_iter(result).into_shared())
    }

    /// Adds a new user.
    #[export]
    fn user_add(&self, _owner: &Node, user: db::User) -> api::Result<()> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.user_add(&user)
    }

    /// Updates the user and all references if its account changes.
    #[export]
    fn user_update(&self, _owner: &Node, account: String, user: db::User) -> api::Result<()> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.user_update(&account, &user)
    }

    /// Deletes the user.
    /// This includes all its borrows & reservations.
    #[export]
    fn user_delete(&self, _owner: &Node, account: String) -> api::Result<()> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.user_delete(&account)
    }

    // Category

    /// Fetches and returns all categories.
    #[export]
    fn category_list(&self, _owner: &Node) -> api::Result<VariantArray> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.category_list()?;
        Ok(VariantArray::from_iter(result).into_shared())
    }

    /// Adds a new category.
    #[export]
    fn category_add(&self, _owner: &Node, category: db::Category) -> api::Result<()> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.category_add(&category)
    }

    /// Updates the category and all references.
    #[export]
    fn category_update(
        &self,
        _owner: &Node,
        id: String,
        category: db::Category,
    ) -> api::Result<()> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.category_update(&id, &category)
    }

    /// Removes the category or returns a `LogicError` if it is still in use.
    #[export]
    fn category_remove(&self, _owner: &Node, id: String) -> api::Result<()> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.category_delete(&id)
    }

    /// Returns the number of books in this category.
    #[export]
    fn category_references(&self, _owner: &Node, id: String) -> api::Result<i64> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.category_references(&id)
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
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.lending_lend(&mut book, &user, days)?;
        Ok(book)
    }

    /// Returns the book.
    #[export]
    fn lending_return(&self, _owner: &Node, mut book: db::Book) -> api::Result<db::Book> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.lending_return(&mut book)?;
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
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.lending_reserve(&mut book, &user)?;
        Ok(book)
    }

    /// Removes the reservation from the specified book.
    #[export]
    fn lending_release(&self, _owner: &Node, mut book: db::Book) -> api::Result<db::Book> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.lending_release(&mut book)?;
        Ok(book)
    }

    /// Returns the list of expired borrowing periods.
    #[export]
    fn lending_overdues(&self, _owner: &Node) -> api::Result<VariantArray> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.lending_overdues()?;
        Ok(VariantArray::from_iter(result.map(|(book, user)| {
            VariantArray::from_iter([book.owned_to_variant(), user.owned_to_variant()].iter())
                .into_shared()
        }))
        .into_shared())
    }

    /// Returns the project settings.
    /// They are fetched when opening a project, so that this function only
    /// returns copies of the cached version.
    #[export]
    fn settings_get(&self, _owner: &Node) -> api::Result<db::Settings> {
        let _timer = debug::timer();
        self.settings.clone().ok_or(Error::NoProject)
    }

    /// Updates project settings.
    #[export]
    fn settings_update(&mut self, _owner: &Node, settings: db::Settings) -> api::Result<()> {
        let _timer = debug::timer();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.settings_update(&settings)?;
        godot_print!("Settings updated");
        // Reload cached settings
        self.settings = Some(db.settings_fetch()?);
        godot_print!("Settings synced");
        Ok(())
    }
}
