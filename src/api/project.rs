use std::iter::FromIterator;

use gdnative::prelude::*;

use crate::api::{self, Error};
use crate::db::{
    self, Database, DatabaseCategory, DatabaseMedium, DatabaseRental, DatabaseSettings,
    DatabaseUser,
};

/// The Global Project Singleton
#[derive(NativeClass, Debug)]
#[inherit(Node)]
pub struct Project {
    db: Option<Database>,
    settings: Option<db::Settings>,
}

struct DebugTimer {
    time: std::time::Instant,
}

impl DebugTimer {
    fn new() -> DebugTimer {
        DebugTimer {
            time: std::time::Instant::now(),
        }
    }
}

impl Drop for DebugTimer {
    fn drop(&mut self) {
        godot_print!("Elapsed time: {}", self.time.elapsed().as_micros());
    }
}

#[methods]
impl Project {
    fn new(_owner: &Node) -> Self {
        Project {
            db: None,
            settings: None,
        }
    }

    /// Opens the specified project and returns if it was successfull.
    #[export]
    fn open(&mut self, _owner: &Node, file: GodotString) -> bool {
        godot_print!("sqlite version: {}", sqlite::version());
        self.db = Database::new(&file.to_string()).ok();
        if let Some(db) = &self.db {
            self.settings = db.settings_fetch().ok();
        } else {
            self.settings = None;
        }
        self.db.is_some()
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

    /// Performes a simple media search with the given `text`.
    #[export]
    fn medium_search(&self, _owner: &Node, text: GodotString) -> api::Result<VariantArray> {
        let _timer = DebugTimer::new();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.medium_search(&text.to_string())?;
        Ok(VariantArray::from_iter(result).into_shared())
    }

    #[export]
    fn medium_search_advanced(
        &self,
        _owner: &Node,
        params: db::MediumSearch,
    ) -> api::Result<VariantArray> {
        let _timer = DebugTimer::new();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.medium_search_advanced(&params)?;
        Ok(VariantArray::from_iter(result).into_shared())
    }

    /// Adds a new medium.
    #[export]
    fn medium_add(&self, _owner: &Node, medium: db::Medium) -> api::Result<()> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.medium_add(&medium)
    }

    /// Updates the medium and all references if its id changes.
    #[export]
    fn medium_update(
        &self,
        _owner: &Node,
        previous_id: String,
        medium: db::Medium,
    ) -> api::Result<()> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.medium_update(&previous_id, &medium)
    }

    /// Deletes the medium including the its authors.
    /// Also borrowers & reservations for this medium are removed.
    #[export]
    fn medium_delete(&self, _owner: &Node, id: String) -> api::Result<()> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.medium_delete(&id)
    }

    /// Generates a new medium id.
    #[export]
    fn medium_generate_id(&self, _owner: &Node, medium: db::Medium) -> api::Result<String> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.medium_generate_id(&medium)
    }

    /// Returns the user with the given `account`.
    #[export]
    fn user_fetch(&self, _owner: &Node, account: GodotString) -> api::Result<db::User> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.user_fetch(&account.to_string())
    }

    /// Performes a simple user search with the given `text`.
    #[export]
    fn user_search(&self, _owner: &Node, text: GodotString) -> api::Result<VariantArray> {
        let _timer = DebugTimer::new();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.user_search(&text.to_string())?;
        Ok(VariantArray::from_iter(result).into_shared())
    }

    /// Adds a new user.
    #[export]
    fn user_add(&self, _owner: &Node, user: db::User) -> api::Result<()> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.user_add(&user)
    }

    /// Updates the user and all references if its account changes.
    #[export]
    fn user_update(&self, _owner: &Node, account: String, user: db::User) -> api::Result<()> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.user_update(&account, &user)
    }

    /// Deletes the user.
    /// This includes all its borrows & reservations.
    #[export]
    fn user_delete(&self, _owner: &Node, account: String) -> api::Result<()> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.user_delete(&account)
    }

    /// Performes a simple user search with the given `text`.
    #[export]
    fn category_list(&self, _owner: &Node) -> api::Result<VariantArray> {
        let _timer = DebugTimer::new();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.category_list()?;
        Ok(VariantArray::from_iter(result).into_shared())
    }

    /// Adds a new category.
    #[export]
    fn category_add(&self, _owner: &Node, category: db::Category) -> api::Result<()> {
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
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.category_update(&id, &category)
    }

    /// Removes the category, assuming it is not referenced anywhere.
    #[export]
    fn category_remove(&self, _owner: &Node, id: String) -> api::Result<()> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.category_delete(&id)
    }

    /// Returns the number of books in this category.
    #[export]
    fn category_references(&self, _owner: &Node, id: String) -> api::Result<i64> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.category_references(&id)
    }

    // Lending
    /// Lends the medium to the specified user.
    #[export]
    fn rental_lend(
        &self,
        _owner: &Node,
        mut medium: db::Medium,
        user: db::User,
        days: i64,
    ) -> api::Result<db::Medium> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.rental_lend(&mut medium, &user, days)?;
        Ok(medium)
    }

    /// Revokes the borrowing when a borrowed medium is returned.
    #[export]
    fn rental_revoke(&self, _owner: &Node, mut medium: db::Medium) -> api::Result<db::Medium> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.rental_revoke(&mut medium)?;
        Ok(medium)
    }

    /// Creates a reservation for the borrowed medium.
    #[export]
    fn rental_reserve(
        &self,
        _owner: &Node,
        mut medium: db::Medium,
        user: db::User,
    ) -> api::Result<db::Medium> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.rental_reserve(&mut medium, &user)?;
        Ok(medium)
    }

    /// Removes the reservation from the specified medium.
    #[export]
    fn rental_release(&self, _owner: &Node, mut medium: db::Medium) -> api::Result<db::Medium> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.rental_release(&mut medium)?;
        Ok(medium)
    }

    /// Returns the list of exceeded borrowing periods.
    #[export]
    fn rental_overdues(&self, _owner: &Node) -> api::Result<VariantArray> {
        let _timer = DebugTimer::new();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.rental_overdues()?;
        Ok(VariantArray::from_iter(result.map(|(medium, user)| {
            VariantArray::from_iter([medium.owned_to_variant(), user.owned_to_variant()].iter())
                .into_shared()
        }))
        .into_shared())
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
        let _timer = DebugTimer::new();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.settings_update(&settings)?;
        // Reload cached settings
        self.settings = Some(db.settings_fetch()?);
        Ok(())
    }
}
