use std::collections::{btree_map::Entry, BTreeMap};

use serde::{Deserialize, Serialize};

use super::Books;
use crate::error::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub section: String,
}

impl Category {
    fn validate(&mut self) -> bool {
        self.id = self.id.trim().to_string();
        self.name = self.name.trim().to_string();
        self.section = self.section.trim().to_string();
        !self.id.is_empty()
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Categories {
    #[serde(flatten)]
    pub data: BTreeMap<String, Category>,
}

impl Categories {
    pub fn list(&self) -> Result<Vec<Category>> {
        Ok(self.data.values().cloned().collect())
    }

    pub fn add(&mut self, mut category: Category) -> Result<Category> {
        if !category.validate() {
            return Err(Error::Arguments);
        }

        match self.data.entry(category.id.clone()) {
            Entry::Vacant(v) => {
                v.insert(category.clone());
                Ok(category)
            }
            _ => Err(Error::InvalidBook),
        }
    }

    pub fn update(
        &mut self,
        id: &str,
        mut category: Category,
        books: &mut Books,
    ) -> Result<Category> {
        let id = id.trim();
        if id.is_empty() || !category.validate() {
            return Err(Error::Arguments);
        }

        if id == category.id {
            if let Some(entry) = self.data.get_mut(id) {
                *entry = category.clone();
                return Ok(category);
            }
        } else {
            if self.data.remove(id).is_some() {
                return match self.data.entry(category.id.clone()) {
                    Entry::Vacant(v) => {
                        v.insert(category.clone());
                        books.update_category_ref(id, &category.id)?;
                        Ok(category)
                    }
                    _ => Err(Error::Arguments),
                };
            }
        }

        Err(Error::NothingFound)
    }

    pub fn delete(&mut self, id: &str, books: &Books) -> Result<()> {
        // Check for books with the category
        for book in books.data.values() {
            if book.category == id {
                return Err(Error::Logic);
            }
        }
        self.data
            .remove(id.trim())
            .map(|_| ())
            .ok_or(Error::NothingFound)
    }
}
