use gdnative::prelude::*;

/// The Database wrapper "class"
#[derive(NativeClass, Debug)]
#[inherit(Object)]
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
    fn new(_owner: &Object) -> Self {
        User {
            account: GodotString::new(),
            forename: GodotString::new(),
            surname: GodotString::new(),
            role: GodotString::new(),
            may_borrow: true,
        }
    }

    #[export]
    pub fn demo(&mut self, _owner: &Object) {
        self.account = "demo.test".into();
        self.forename = "Demo".into();
        self.surname = "Test".into();
        self.role = "Admin".into();
        self.may_borrow = true;
    }

    #[export]
    fn list_item(&mut self, _owner: &Object) -> StringArray {
        StringArray::from_vec(vec![
            self.account.clone(),
            self.forename.clone(),
            self.surname.clone(),
            self.role.clone(),
        ])
    }
}
