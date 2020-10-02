use std::iter::FromIterator;
use std::time::Instant;

use gdnative::prelude::*;

use crate::api::{self, Error};
use crate::db::{Database, DatabaseCategory, DatabaseMedium, DatabaseRental, DatabaseUser};

/// The Global Project Singleton
#[derive(NativeClass, Debug)]
#[inherit(Node)]
pub struct Project {
    db: Option<Database>,
}

#[methods]
impl Project {
    fn new(_owner: &Node) -> Self {
        Project { db: None }
    }

    /// Opens the specified project and returns if it was successfull.
    #[export]
    fn open(&mut self, _owner: &Node, file: GodotString) -> bool {
        godot_print!("sqlite version: {}", sqlite::version());
        godot_print!("opening {}", file);
        self.db = Database::new(&file.to_string()).ok();
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
        self.db = None
    }

    /// Performes a simple media search with the given `text`.
    #[export]
    fn medium_search(&self, _owner: &Node, text: GodotString) -> api::Result<VariantArray> {
        let timer = Instant::now();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.medium_search(&text.to_string())?;
        let result = Ok(VariantArray::from_iter(result.map(|x| {
            let instance = api::Medium::new_instance();
            instance.map_mut(|u, _| u.fill(x)).unwrap();
            instance.owned_to_variant()
        }))
        .into_shared());
        godot_print!("access time: {}ms", timer.elapsed().as_millis());
        result
    }

    /// Adds a new medium.
    #[export]
    fn medium_add(&self, _owner: &Node, medium: Ref<Reference>) -> api::Result<()> {
        let medium = Instance::<api::Medium, Unique>::from_base(unsafe { medium.assume_unique() })
            .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        medium.map(|medium, _| db.medium_add(&medium.db())).unwrap()
    }

    /// Updates the medium and all references if its id changes.
    #[export]
    fn medium_update(
        &self,
        _owner: &Node,
        previous_id: GodotString,
        medium: Ref<Reference>,
    ) -> api::Result<()> {
        let medium = Instance::<api::Medium, Unique>::from_base(unsafe { medium.assume_unique() })
            .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        medium
            .map(|medium, _| db.medium_update(&previous_id.to_string(), &medium.db()))
            .unwrap()
    }

    /// Deletes the medium including the its authors.
    /// Also borrowers & reservations for this medium are removed.
    #[export]
    fn medium_delete(&self, _owner: &Node, id: GodotString) -> api::Result<()> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.medium_delete(&id.to_string())
    }
    #[export]
    fn medium_generate_id(
        &self,
        _owner: &Node,
        medium: Ref<Reference>,
    ) -> api::Result<GodotString> {
        let medium = Instance::<api::Medium, Unique>::from_base(unsafe { medium.assume_unique() })
            .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        medium
            .map(|medium, _| db.medium_generate_id(&medium.db()).map(|x| x.into()))
            .unwrap()
    }

    /// Performes a simple user search with the given `text`.
    #[export]
    fn user_search(&self, _owner: &Node, text: GodotString) -> api::Result<VariantArray> {
        let timer = Instant::now();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.user_search(&text.to_string())?;
        let result = Ok(VariantArray::from_iter(result.map(|x| {
            let instance = api::User::new_instance();
            instance.map_mut(|m, _| m.fill(x)).unwrap();
            instance.owned_to_variant()
        }))
        .into_shared());
        godot_print!("access time: {}ms", timer.elapsed().as_millis());
        result
    }

    /// Adds a new user.
    #[export]
    fn user_add(&self, _owner: &Node, user: Ref<Reference>) -> api::Result<()> {
        let user = Instance::<api::User, Unique>::from_base(unsafe { user.assume_unique() })
            .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        user.map(|user, _| db.user_add(&user.db())).unwrap()
    }

    /// Updates the user and all references if its account changes.
    #[export]
    fn user_update(
        &self,
        _owner: &Node,
        account: GodotString,
        user: Ref<Reference>,
    ) -> api::Result<()> {
        let user = Instance::<api::User, Unique>::from_base(unsafe { user.assume_unique() })
            .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        user.map(|user, _| db.user_update(&account.to_string(), &user.db()))
            .unwrap()
    }

    /// Deletes the user.
    /// This includes all its borrows & reservations.
    #[export]
    fn user_delete(&self, _owner: &Node, account: GodotString) -> api::Result<()> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.user_delete(&account.to_string())
    }

    /// Performes a simple user search with the given `text`.
    #[export]
    fn category_list(&self, _owner: &Node) -> api::Result<VariantArray> {
        let timer = Instant::now();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.category_list()?;
        let result = Ok(VariantArray::from_iter(result.map(|x| {
            let instance = api::Category::new_instance();
            instance.map_mut(|c, _| c.fill(x)).unwrap();
            instance.owned_to_variant()
        }))
        .into_shared());
        godot_print!("access time: {}ms", timer.elapsed().as_millis());
        result
    }

    /// Adds a new category.
    #[export]
    fn category_add(&self, _owner: &Node, category: Ref<Reference>) -> api::Result<()> {
        let category =
            Instance::<api::Category, Unique>::from_base(unsafe { category.assume_unique() })
                .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        category
            .map(|category, _| db.category_add(&category.db()))
            .unwrap()
    }

    /// Updates the category and all references.
    #[export]
    fn category_update(
        &self,
        _owner: &Node,
        id: GodotString,
        category: Ref<Reference>,
    ) -> api::Result<()> {
        let category =
            Instance::<api::Category, Unique>::from_base(unsafe { category.assume_unique() })
                .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        category
            .map(|category, _| db.category_update(&id.to_string(), &category.db()))
            .unwrap()
    }

    /// Removes the category, assuming it is not referenced anywhere.
    #[export]
    fn category_remove(&self, _owner: &Node, id: GodotString) -> api::Result<()> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.category_delete(&id.to_string())
    }

    /// Returns the number of books in this category.
    #[export]
    fn category_references(&self, _owner: &Node, id: GodotString) -> api::Result<i64> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.category_references(&id.to_string())
    }

    // Lending
    /// Lends the medium to the specified user.
    #[export]
    fn rental_lend(
        &self,
        _owner: &Node,
        medium: GodotString,
        user: GodotString,
        days: i64,
    ) -> api::Result<()> {
        let until = chrono::Utc::today() + chrono::Duration::days(days);
        godot_print!("Lend {} to {} until {}", medium, user, until.format("%F"));
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.rental_lend(
            &medium.to_string(),
            &user.to_string(),
            &until.format("%F").to_string(),
        )
    }

    /// Revokes the borrowing when a borrowed medium is returned.
    #[export]
    fn rental_revoke(&self, _owner: &Node, medium: GodotString) -> api::Result<()> {
        godot_print!("revoke {}", medium);
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.rental_revoke(&medium.to_string())
    }

    /// Creates a reservation for the borrowed medium.
    #[export]
    fn rental_reserve(
        &self,
        _owner: &Node,
        medium: GodotString,
        user: GodotString,
    ) -> api::Result<()> {
        godot_print!("reserve {} for {}", medium, user);
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.rental_reserve(&medium.to_string(), &user.to_string())
    }

    /// Removes the reservation from the specified medium.
    #[export]
    fn rental_release(&self, _owner: &Node, medium: GodotString) -> api::Result<()> {
        godot_print!("delete reservation {} ", medium);
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.rental_release(&medium.to_string())
    }
}
