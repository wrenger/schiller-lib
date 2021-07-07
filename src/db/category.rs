use std::collections::HashMap;

use crate::api;

use super::raw::DatabaseExt;
use super::{DBIter, Database, ReadStmt};

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

#[derive(Debug, Clone, gdnative::ToVariant, gdnative::FromVariant)]
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

impl ReadStmt for Category {
    type Error = api::Error;

    fn read(
        stmt: &sqlite::Statement<'_>,
        columns: &HashMap<String, usize>,
    ) -> api::Result<Category> {
        Ok(Category {
            id: stmt.read(columns["id"])?,
            name: stmt.read(columns["name"])?,
            section: stmt.read(columns["section"])?,
        })
    }
}

/// Returns all categories.
pub fn list(db: &Database) -> api::Result<DBIter<Category>> {
    let stmt = db.db.prepare(LIST)?;
    Ok(DBIter::new(stmt))
}

/// Adds a new category.
pub fn add(db: &Database, category: &Category) -> api::Result<()> {
    if !category.is_valid() {
        return Err(api::Error::InvalidArguments);
    }

    let mut stmt = db.db.prepare(ADD)?;
    stmt.bind(1, category.id.trim())?;
    stmt.bind(2, category.name.trim())?;
    stmt.bind(3, category.section.trim())?;
    if stmt.next()? != sqlite::State::Done {
        return Err(api::Error::SQLError);
    }
    Ok(())
}

/// Updates the category and all references.
pub fn update(db: &Database, id: &str, category: &Category) -> api::Result<()> {
    if !category.is_valid() {
        return Err(api::Error::InvalidArguments);
    }

    let transaction = db.db.transaction()?;
    // Update category
    let mut stmt = db.db.prepare(UPDATE)?;
    stmt.bind(1, category.id.trim())?;
    stmt.bind(2, category.name.trim())?;
    stmt.bind(3, category.section.trim())?;
    stmt.bind(4, id)?;
    if stmt.next()? != sqlite::State::Done {
        return Err(api::Error::SQLError);
    }

    if id != category.id {
        // Update category ids of related media
        let mut stmt = db.db.prepare(UPDATE_MEDIA)?;
        stmt.bind(1, category.id.trim())?;
        stmt.bind(2, id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
    }

    transaction.commit()?;
    Ok(())
}

/// Removes the category, assuming it is not referenced anywhere.
pub fn delete(db: &Database, id: &str) -> api::Result<()> {
    let id = id.trim();
    if id.is_empty() {
        return Err(api::Error::InvalidArguments);
    }

    let transaction = db.db.transaction()?;
    // Do not allow the removal of used categories
    if references(db, id)? > 0 {
        return Err(api::Error::LogicError);
    }

    let mut stmt = db.db.prepare(DELETE)?;
    stmt.bind(1, id)?;
    if stmt.next()? != sqlite::State::Done {
        return Err(api::Error::SQLError);
    }

    transaction.commit()?;
    Ok(())
}

/// Returns the number of books in this category.
pub fn references(db: &Database, id: &str) -> api::Result<i64> {
    let id = id.trim();
    if id.is_empty() {
        return Err(api::Error::InvalidArguments);
    }

    let mut stmt = db.db.prepare(REFERENCED)?;
    stmt.bind(1, id)?;
    if stmt.next()? != sqlite::State::Row {
        return Err(api::Error::SQLError);
    }
    Ok(stmt.read(0)?)
}
