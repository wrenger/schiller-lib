use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use gluer::metadata;
use serde::{Deserialize, Serialize};

use super::Books;
use crate::error::{Error, Result};

/// Data object for categories
#[metadata]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub section: String,
}

impl Category {
    /// Check if category is valid
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
            _ => Err(Error::Arguments),
        }
    }

    /// Update the category with the given id
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
        } else if self.data.contains_key(id) {
            return match self.data.entry(category.id.clone()) {
                Entry::Vacant(v) => {
                    v.insert(category.clone());
                    books.update_category(id, &category.id)?;
                    self.data.remove(id);
                    Ok(category)
                }
                _ => Err(Error::Arguments),
            };
        }

        Err(Error::NothingFound)
    }

    pub fn delete(&mut self, id: &str, books: &Books) -> Result<()> {
        let id = id.trim();
        if id.is_empty() {
            return Err(Error::Arguments);
        }

        // Check for books with the category
        for book in books.data.values() {
            if book.category == id {
                return Err(Error::ReferencedCategory);
            }
        }
        self.data.remove(id).map(|_| ()).ok_or(Error::NothingFound)
    }
}
