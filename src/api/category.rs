use gdnative::prelude::*;

use crate::db::DBCategory;

/// The Category wrapper "class"
#[derive(NativeClass, Debug)]
#[inherit(Reference)]
pub struct Category {
    #[property]
    pub id: GodotString,
    #[property]
    pub name: GodotString,
    #[property]
    pub section: GodotString,
}

#[methods]
impl Category {
    fn new(_owner: &Reference) -> Self {
        Category {
            id: GodotString::new(),
            name: GodotString::new(),
            section: GodotString::new(),
        }
    }

    pub fn fill(&mut self, category: DBCategory) {
        self.id = category.id.into();
        self.name = category.name.into();
        self.section = category.section.into();
    }

    pub fn db(&self) -> DBCategory {
        DBCategory {
            id: self.id.to_string(),
            name: self.name.to_string(),
            section: self.section.to_string(),
        }
    }
}
