use std::iter::FromIterator;
use std::time::Instant;

use gdnative::prelude::*;

use crate::api::{self, Error};
use crate::db::Database;

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

    // Closes the connection to the projects database.
    #[export]
    fn close(&mut self, _owner: &Node) {
        self.db = None
    }

    /// Performes a simple media search with the given `text`.
    #[export]
    fn search_media(&self, _owner: &Node, text: GodotString) -> api::Result<VariantArray> {
        let timer = Instant::now();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.search_media(&text.to_string())?;
        let result = Ok(VariantArray::from_iter(result.map(|x| {
            let instance = api::Medium::new_instance();
            instance.map_mut(|u, _| u.fill(x)).unwrap();
            instance.owned_to_variant()
        }))
        .into_shared());
        godot_print!("access time: {}ms", timer.elapsed().as_millis());
        result
    }

    /// Performes a simple user search with the given `text`.
    #[export]
    fn search_users(&self, _owner: &Node, text: GodotString) -> api::Result<VariantArray> {
        let timer = Instant::now();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.search_users(&text.to_string())?;
        let result = Ok(VariantArray::from_iter(result.map(|x| {
            let instance = api::User::new_instance();
            instance.map_mut(|m, _| m.fill(x)).unwrap();
            instance.owned_to_variant()
        }))
        .into_shared());
        godot_print!("access time: {}ms", timer.elapsed().as_millis());
        result
    }

    /// Performes a simple user search with the given `text`.
    #[export]
    fn categories(&self, _owner: &Node) -> api::Result<VariantArray> {
        let timer = Instant::now();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.categories()?;
        let result = Ok(VariantArray::from_iter(result.map(|x| {
            let instance = api::Category::new_instance();
            instance.map_mut(|c, _| c.fill(x)).unwrap();
            instance.owned_to_variant()
        }))
        .into_shared());
        godot_print!("access time: {}ms", timer.elapsed().as_millis());
        result
    }

    /// Adds a new medium.
    #[export]
    fn add_medium(&self, _owner: &Node, medium: Ref<Reference>) -> api::Result<()> {
        let medium = Instance::<api::Medium, Unique>::from_base(unsafe { medium.assume_unique() })
            .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        medium.map(|medium, _| db.add_medium(&medium.db())).unwrap()
    }

    /// Updates the medium and all references if its id changes.
    #[export]
    fn update_medium(
        &self,
        _owner: &Node,
        previous_id: GodotString,
        medium: Ref<Reference>,
    ) -> api::Result<()> {
        let medium = Instance::<api::Medium, Unique>::from_base(unsafe { medium.assume_unique() })
            .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        medium
            .map(|medium, _| db.update_medium(&previous_id.to_string(), &medium.db()))
            .unwrap()
    }

    /// Deletes the medium including the its authors.
    /// Also borrowers & reservations for this medium are removed.
    #[export]
    fn delete_medium(&self, _owner: &Node, id: GodotString) -> api::Result<()> {
        let timer = Instant::now();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.delete_medium(&id.to_string())?;
        godot_print!("access time: {}ms", timer.elapsed().as_millis());
        Ok(())
    }

    /// Adds a new user.
    #[export]
    fn add_user(&self, _owner: &Node, user: Ref<Reference>) -> api::Result<()> {
        let user = Instance::<api::User, Unique>::from_base(unsafe { user.assume_unique() })
            .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        user.map(|user, _| db.add_user(&user.db())).unwrap()
    }

    /// Updates the user and all references if its account changes.
    #[export]
    fn update_user(
        &self,
        _owner: &Node,
        previous_account: GodotString,
        user: Ref<Reference>,
    ) -> api::Result<()> {
        let user = Instance::<api::User, Unique>::from_base(unsafe { user.assume_unique() })
            .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        user.map(|user, _| db.update_user(&previous_account.to_string(), &user.db()))
            .unwrap()
    }

    /// Deletes the user.
    /// This includes all its borrows & reservations.
    #[export]
    fn delete_user(&self, _owner: &Node, account: GodotString) -> api::Result<()> {
        let timer = Instant::now();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.delete_user(&account.to_string())?;
        godot_print!("access time: {}ms", timer.elapsed().as_millis());
        Ok(())
    }

    // Lending
    /// Lends the medium to the specified user.
    #[export]
    fn lend(
        &self,
        _owner: &Node,
        medium: GodotString,
        user: GodotString,
        days: i64,
    ) -> api::Result<()> {
        let until = chrono::Utc::today() + chrono::Duration::days(days);
        godot_print!("Lend {} to {} until {}", medium, user, until.format("%F"));
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.lend(
            &medium.to_string(),
            &user.to_string(),
            &until.format("%F").to_string(),
        )
    }

    /// Revokes the borrowing when a borrowed medium is returned.
    #[export]
    fn revoke(&self, _owner: &Node, medium: GodotString) -> api::Result<()> {
        godot_print!("revoke {}", medium);
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.revoke(&medium.to_string())
    }

    /// Creates a reservation for the borrowed medium.
    #[export]
    fn reserve(&self, _owner: &Node, medium: GodotString, user: GodotString) -> api::Result<()> {
        godot_print!("reserve {} for {}", medium, user);
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.reserve(&medium.to_string(), &user.to_string())
    }

    /// Removes the reservation from the specified medium.
    #[export]
    fn release(&self, _owner: &Node, medium: GodotString) -> api::Result<()> {
        godot_print!("delete reservation {} ", medium);
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.release(&medium.to_string())
    }
}
