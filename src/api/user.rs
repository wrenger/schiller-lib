use gdnative::prelude::*;

use crate::db::DBUser;

/// The User wrapper "class"
#[derive(NativeClass, Debug)]
#[inherit(Reference)]
pub struct User {
    #[property]
    pub account: GodotString,
    #[property]
    pub forename: GodotString,
    #[property]
    pub surname: GodotString,
    #[property]
    pub role: GodotString,
    #[property]
    pub may_borrow: bool,
}

#[methods]
impl User {
    fn new(_owner: &Reference) -> Self {
        User {
            account: GodotString::new(),
            forename: GodotString::new(),
            surname: GodotString::new(),
            role: GodotString::new(),
            may_borrow: true,
        }
    }

    pub fn fill(&mut self, user: DBUser) {
        self.account = user.account.into();
        self.forename = user.forename.into();
        self.surname = user.surname.into();
        self.role = user.role.into();
        self.may_borrow = user.may_borrow;
    }

    pub fn db(&self) -> DBUser {
        DBUser {
            account: self.account.to_string(),
            forename: self.forename.to_string(),
            surname: self.surname.to_string(),
            role: self.role.to_string(),
            may_borrow: self.may_borrow,
        }
    }

    #[export]
    fn list_item(&mut self, _owner: &Reference) -> StringArray {
        StringArray::from_vec(vec![
            self.account.clone(),
            self.forename.clone(),
            self.surname.clone(),
            self.role.clone(),
        ])
    }
}
