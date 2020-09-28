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

    #[export]
    fn open(&mut self, _owner: &Node, file: GodotString) -> bool {
        godot_print!("opening {}", file);
        self.db = Database::new(&file.to_string()).ok();
        self.db.is_some()
    }

    // Medium

    #[export]
    fn search_media_basic(&self, _owner: &Node, text: GodotString) -> api::Result<VariantArray> {
        let timer = Instant::now();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.search_media_basic(&text.to_string())?;
        let result = Ok(VariantArray::from_iter(result.map(|x| {
            let instance = api::Medium::new_instance();
            instance.map_mut(|u, _| u.fill(x)).unwrap();
            instance.owned_to_variant()
        }))
        .into_shared());
        godot_print!("access time: {}ms", timer.elapsed().as_millis());
        result
    }
    #[export]
    fn update_medium(
        &self,
        _owner: &Node,
        previous_id: GodotString,
        medium: Ref<Object>,
    ) -> api::Result<()> {
        let medium = Instance::<api::Medium, Unique>::from_base(unsafe { medium.assume_unique() })
            .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        medium
            .map(|medium, _| db.update_medium(&previous_id.to_string(), medium))
            .unwrap()
    }
    #[export]
    fn delete_medium(&self, _owner: &Node, id: GodotString) -> api::Result<()> {
        let timer = Instant::now();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.delete_medium(&id.to_string())?;
        godot_print!("access time: {}ms", timer.elapsed().as_millis());
        Ok(())
    }

    // User

    #[export]
    fn search_user_basic(&self, _owner: &Node, text: GodotString) -> api::Result<VariantArray> {
        let timer = Instant::now();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        let result = db.search_user_basic(&text.to_string())?;
        let result = Ok(VariantArray::from_iter(result.map(|x| {
            let instance = api::User::new_instance();
            instance.map_mut(|m, _| m.fill(x)).unwrap();
            instance.owned_to_variant()
        }))
        .into_shared());
        godot_print!("access time: {}ms", timer.elapsed().as_millis());
        result
    }
    #[export]
    fn update_user(
        &self,
        _owner: &Node,
        previous_account: GodotString,
        user: Ref<Object>,
    ) -> api::Result<()> {
        let user = Instance::<api::User, Unique>::from_base(unsafe { user.assume_unique() })
            .ok_or(Error::InvalidArguments)?;
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        user.map(|user, _| db.update_user(&previous_account.to_string(), user))
            .unwrap()
    }
    #[export]
    fn delete_user(&self, _owner: &Node, account: GodotString) -> api::Result<()> {
        let timer = Instant::now();
        let db = self.db.as_ref().ok_or(Error::NoProject)?;
        db.delete_user(&account.to_string())?;
        godot_print!("access time: {}ms", timer.elapsed().as_millis());
        Ok(())
    }

    // Lending

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
        Ok(())
    }
    #[export]
    fn revoke(&self, _owner: &Node, medium: GodotString) -> api::Result<()> {
        godot_print!("revoke {}", medium);
        Ok(())
    }
    #[export]
    fn reserve(&self, _owner: &Node, medium: GodotString, user: GodotString) -> api::Result<()> {
        godot_print!("reserve {} for {}", medium, user);
        Ok(())
    }
    #[export]
    fn delete_reservation(&self, _owner: &Node, medium: GodotString) -> api::Result<()> {
        godot_print!("delete reservation {} ", medium);
        Ok(())
    }
}
