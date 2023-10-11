use serde::{Serialize, Deserialize};

use crate::error::{Error, Result};

use super::{DBIter, Database, FromRow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub section: String,
}

impl Category {
    fn is_valid(&self) -> bool {
        !self.id.trim().is_empty()
            && !self.name.trim().is_empty()
            && !self.section.trim().is_empty()
    }
}

impl FromRow for Category {
    fn from_row(rows: &rusqlite::Row) -> rusqlite::Result<Category> {
        Ok(Category {
            id: rows.get("id")?,
            name: rows.get("name")?,
            section: rows.get("section")?,
        })
    }
}

/// Returns all categories.
pub fn list(db: &Database) -> Result<Vec<Category>> {
    let mut stmt = db
        .con
        .prepare("select id, name, section from category order by section, id")?;
    let rows = stmt.query([])?;
    DBIter::new(rows).collect()
}

/// Adds a new category.
pub fn add(db: &Database, category: &Category) -> Result<()> {
    if !category.is_valid() {
        return Err(Error::Arguments);
    }

    db.con.execute(
        "insert into category values (?, ?, ?)",
        [
            category.id.trim(),
            category.name.trim(),
            category.section.trim(),
        ],
    )?;
    Ok(())
}

/// Updates the category and all references.
pub fn update(db: &Database, id: &str, category: &Category) -> Result<()> {
    if !category.is_valid() {
        return Err(Error::Arguments);
    }

    let transaction = db.transaction()?;
    // Update category
    transaction.execute(
        "update category set id=?, name=?, section=? where id=?",
        [
            category.id.trim(),
            category.name.trim(),
            category.section.trim(),
            id,
        ],
    )?;

    if id != category.id {
        // Update category ids of related media
        transaction.execute(
            "update medium set category=? where category=?",
            [category.id.trim(), id],
        )?;
    }

    transaction.commit()?;
    Ok(())
}

/// Removes the category, assuming it is not referenced anywhere.
pub fn delete(db: &Database, id: &str) -> Result<()> {
    let id = id.trim();
    if id.is_empty() {
        return Err(Error::Arguments);
    }

    let transaction = db.transaction()?;
    // Do not allow the removal of used categories
    if references(db, id)? > 0 {
        return Err(Error::Logic);
    }

    transaction.execute("delete from category where id=?", [id])?;

    transaction.commit()?;
    Ok(())
}

/// Returns the number of books in this category.
pub fn references(db: &Database, id: &str) -> Result<i64> {
    let id = id.trim();
    if id.is_empty() {
        return Err(Error::Arguments);
    }

    Ok(db.con.query_row(
        "select count(id) from medium where category=?",
        [id],
        |row| row.get::<usize, i64>(0),
    )?)
}
