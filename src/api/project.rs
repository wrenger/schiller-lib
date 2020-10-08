use std::iter::FromIterator;

use gdnative::prelude::*;

use crate::api::{self, Error};
use crate::db::{
    Database, DatabaseCategory, DatabaseMedium, DatabaseRental, DatabaseSettings, DatabaseUser,
};

/// The Global Project Singleton
#[derive(NativeClass, Debug)]
#[inherit(Node)]
pub struct Project {
    db: Option<Database>,
    settings: Option<Instance<api::Settings, Shared>>,
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
            if let Ok(settings) = db.settings_fetch() {
                self.settings = Some(api::Settings::db_instance(settings).into_shared());
            }
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
        Ok(
            VariantArray::from_iter(result.map(|x| api::Medium::db_instance(x).owned_to_variant()))
                .into_shared(),
        )
    }

    #[export]
    fn medium_search_advanced(
        &self,
        _owner: &Node,
        id: GodotString,
        isbn: GodotString,
        title: GodotString,
        publisher: GodotString,
        authors: GodotString,
        year: GodotString,
        category: GodotString,
        note: GodotString,
        user: GodotString,
        state: i64,
    ) -> api::Result<VariantArray> {
        let _timer = DebugTimer::new();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.medium_search_advanced(
            &id.to_string(),
            &isbn.to_string(),
            &title.to_string(),
            &publisher.to_string(),
            &authors.to_string(),
            &year.to_string(),
            &category.to_string(),
            &note.to_string(),
            &user.to_string(),
            state.into(),
        )?;
        Ok(
            VariantArray::from_iter(result.map(|x| api::Medium::db_instance(x).owned_to_variant()))
                .into_shared(),
        )
    }

    /// Adds a new medium.
    #[export]
    fn medium_add(&self, _owner: &Node, medium: Ref<Reference>) -> api::Result<()> {
        let medium = Instance::<api::Medium, Unique>::from_base(unsafe { medium.assume_unique() })
            .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        medium.map(|m, _| db.medium_add(&m.db())).unwrap()
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
            .map(|m, _| db.medium_update(&previous_id.to_string(), &m.db()))
            .unwrap()
    }

    /// Deletes the medium including the its authors.
    /// Also borrowers & reservations for this medium are removed.
    #[export]
    fn medium_delete(&self, _owner: &Node, id: GodotString) -> api::Result<()> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.medium_delete(&id.to_string())
    }

    /// Generates a new medium id.
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
            .map(|m, _| db.medium_generate_id(&m.db()).map(|x| x.into()))
            .unwrap()
    }

    /// Returns the user with the given `account`.
    #[export]
    fn user_fetch(
        &self,
        _owner: &Node,
        account: GodotString,
    ) -> api::Result<Instance<api::User, Shared>> {
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.user_fetch(&account.to_string())
            .map(|u| api::User::db_instance(u).into_shared())
    }

    /// Performes a simple user search with the given `text`.
    #[export]
    fn user_search(&self, _owner: &Node, text: GodotString) -> api::Result<VariantArray> {
        let _timer = DebugTimer::new();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.user_search(&text.to_string())?;
        Ok(
            VariantArray::from_iter(result.map(|x| api::User::db_instance(x).owned_to_variant()))
                .into_shared(),
        )
    }

    /// Adds a new user.
    #[export]
    fn user_add(&self, _owner: &Node, user: Ref<Reference>) -> api::Result<()> {
        let user = Instance::<api::User, Unique>::from_base(unsafe { user.assume_unique() })
            .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        user.map(|u, _| db.user_add(&u.db())).unwrap()
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
        user.map(|u, _| db.user_update(&account.to_string(), &u.db()))
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
        let _timer = DebugTimer::new();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.category_list()?;
        Ok(VariantArray::from_iter(result.map(|x| {
            let instance = api::Category::new_instance();
            instance.map_mut(|c, _| c.fill(x)).unwrap();
            instance.owned_to_variant()
        }))
        .into_shared())
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
        medium: Ref<Reference>,
        user: Ref<Reference>,
        days: i64,
    ) -> api::Result<Instance<api::Medium, Shared>> {
        let mut medium =
            Instance::<api::Medium, Unique>::from_base(unsafe { medium.assume_unique() })
                .ok_or(Error::InvalidArguments)?
                .map(|m, _| m.db())
                .unwrap();
        let user = Instance::<api::User, Unique>::from_base(unsafe { user.assume_unique() })
            .ok_or(Error::InvalidArguments)?
            .map(|u, _| u.db())
            .unwrap();

        let db = self.db.as_ref().ok_or(Error::NoProject)?;

        db.rental_lend(&mut medium, &user, days)?;
        Ok(api::Medium::db_instance(medium).into_shared())
    }

    /// Revokes the borrowing when a borrowed medium is returned.
    #[export]
    fn rental_revoke(
        &self,
        _owner: &Node,
        medium: Ref<Reference>,
    ) -> api::Result<Instance<api::Medium, Shared>> {
        let mut medium =
            Instance::<api::Medium, Unique>::from_base(unsafe { medium.assume_unique() })
                .ok_or(Error::InvalidArguments)?
                .map(|m, _| m.db())
                .unwrap();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        if medium.borrower.is_empty() {
            return Err(api::Error::LogicError);
        }

        db.rental_revoke(&mut medium)?;
        Ok(api::Medium::db_instance(medium).into_shared())
    }

    /// Creates a reservation for the borrowed medium.
    #[export]
    fn rental_reserve(
        &self,
        _owner: &Node,
        medium: Ref<Reference>,
        user: Ref<Reference>,
    ) -> api::Result<Instance<api::Medium, Shared>> {
        let mut medium =
            Instance::<api::Medium, Unique>::from_base(unsafe { medium.assume_unique() })
                .ok_or(Error::InvalidArguments)?
                .map(|m, _| m.db())
                .unwrap();
        let user = Instance::<api::User, Unique>::from_base(unsafe { user.assume_unique() })
            .ok_or(Error::InvalidArguments)?
            .map(|m, _| m.db())
            .unwrap();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;

        db.rental_reserve(&mut medium, &user)?;
        Ok(api::Medium::db_instance(medium).into_shared())
    }

    /// Removes the reservation from the specified medium.
    #[export]
    fn rental_release(
        &self,
        _owner: &Node,
        medium: Ref<Reference>,
    ) -> api::Result<Instance<api::Medium, Shared>> {
        let mut medium =
            Instance::<api::Medium, Unique>::from_base(unsafe { medium.assume_unique() })
                .ok_or(Error::InvalidArguments)?
                .map(|m, _| m.db())
                .unwrap();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        if medium.reservation.is_empty() {
            return Err(api::Error::LogicError);
        }

        db.rental_release(&mut medium)?;
        Ok(api::Medium::db_instance(medium).into_shared())
    }

    /// Returns the list of exceeded borrowing periods.
    #[export]
    fn rental_overdues(&self, _owner: &Node) -> api::Result<VariantArray> {
        let _timer = DebugTimer::new();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.rental_overdues()?;
        Ok(VariantArray::from_iter(result.map(|(medium, user)| {
            let medium = api::Medium::db_instance(medium).owned_to_variant();
            let user = api::User::db_instance(user).owned_to_variant();
            VariantArray::from_iter([medium, user].iter()).into_shared()
        }))
        .into_shared())
    }

    /// Returns the project settings.
    /// They are fetched when opening a project, so that this function only
    /// returns copies of the cached version.
    #[export]
    fn settings_get(&self, _owner: &Node) -> api::Result<Instance<api::Settings, Shared>> {
        self.settings.clone().ok_or(Error::NoProject)
    }

    /// Updates project settings.
    #[export]
    fn settings_update(&mut self, _owner: &Node, settings: Ref<Reference>) -> api::Result<()> {
        let _timer = DebugTimer::new();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let settings =
            Instance::<api::Settings, Unique>::from_base(unsafe { settings.assume_unique() })
                .ok_or(Error::InvalidArguments)?
                .map(|m, _| m.db())
                .unwrap();
        db.settings_update(&settings)?;
        // Reload cached settings
        self.settings = Some(api::Settings::db_instance(db.settings_fetch()?).into_shared());
        Ok(())
    }
}
