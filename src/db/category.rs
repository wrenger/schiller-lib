use crate::api;

use super::{DBIter, Database, FromRow};

use gdnative::derive::{FromVariant, ToVariant};

const LIST: &str = "\
    select id, name, section from category order by section, id \
";
const ADD: &str = "\
    insert into category values (?, ?, ?) \
";
const UPDATE: &str = "\
    update category set id=?, name=?, section=? where id=? \
";
const UPDATE_MEDIA: &str = "\
    update medium set category=? where category=? \
";
const DELETE: &str = "\
    delete from category where id=? \
";
const REFERENCED: &str = "\
    select count(id) from medium where category=? \
";

#[derive(Debug, Clone, ToVariant, FromVariant)]
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
pub fn list(db: &Database) -> api::Result<Vec<Category>> {
    let mut stmt = db.con.prepare(LIST)?;
    let rows = stmt.query([])?;
    DBIter::new(rows).collect()
}

/// Adds a new category.
pub fn add(db: &Database, category: &Category) -> api::Result<()> {
    if !category.is_valid() {
        return Err(api::Error::Arguments);
    }

    db.con.execute(
        ADD,
        [
            category.id.trim(),
            category.name.trim(),
            category.section.trim(),
        ],
    )?;
    Ok(())
}

/// Updates the category and all references.
pub fn update(db: &Database, id: &str, category: &Category) -> api::Result<()> {
    if !category.is_valid() {
        return Err(api::Error::Arguments);
    }

    let transaction = db.transaction()?;
    // Update category
    transaction.execute(
        UPDATE,
        [
            category.id.trim(),
            category.name.trim(),
            category.section.trim(),
            id,
        ],
    )?;

    if id != category.id {
        // Update category ids of related media
        transaction.execute(UPDATE_MEDIA, [category.id.trim(), id])?;
    }

    transaction.commit()?;
    Ok(())
}

/// Removes the category, assuming it is not referenced anywhere.
pub fn delete(db: &Database, id: &str) -> api::Result<()> {
    let id = id.trim();
    if id.is_empty() {
        return Err(api::Error::Arguments);
    }

    let transaction = db.transaction()?;
    // Do not allow the removal of used categories
    if references(db, id)? > 0 {
        return Err(api::Error::Logic);
    }

    transaction.execute(DELETE, [id])?;

    transaction.commit()?;
    Ok(())
}

/// Returns the number of books in this category.
pub fn references(db: &Database, id: &str) -> api::Result<i64> {
    let id = id.trim();
    if id.is_empty() {
        return Err(api::Error::Arguments);
    }

    Ok(db
        .con
        .query_row(REFERENCED, [id], |row| row.get::<usize, i64>(0))?)
}
