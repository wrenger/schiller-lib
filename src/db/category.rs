use crate::api;

use super::raw::DatabaseExt;
use super::{DBIter, ReadStmt};

const LIST: &str = r#"
select * from category order by section, id
"#;

const ADD: &str = r#"
insert into category values (?, ?, ?)
"#;

const UPDATE: &str = r#"
update category set id=?, name=?, section=? where id=?
"#;

const UPDATE_MEDIA: &str = r#"
update medium set category=? where category=?
"#;

const DELETE: &str = r#"
delete from category where id=?
"#;

const REFERENCED: &str = r#"
select count(id) from medium where category=?
"#;

#[derive(Debug)]
pub struct DBCategory {
    pub id: String,
    pub name: String,
    pub section: String,
}

impl ReadStmt for DBCategory {
    type Error = api::Error;

    fn read(stmt: &sqlite::Statement<'_>) -> api::Result<DBCategory> {
        Ok(DBCategory {
            id: stmt.read(0)?,
            name: stmt.read(1)?,
            section: stmt.read(2)?,
        })
    }
}

pub trait DatabaseCategory {
    fn db(&self) -> &sqlite::Connection;

    /// Returns all categories.
    fn category_list(&self) -> api::Result<DBIter<DBCategory>> {
        let stmt = self.db().prepare(LIST)?;
        Ok(DBIter::new(stmt))
    }

    /// Adds a new category.
    fn category_add(&self, category: &DBCategory) -> api::Result<()> {
        let mut stmt = self.db().prepare(ADD)?;
        stmt.bind(1, category.id.as_str())?;
        stmt.bind(2, category.name.as_str())?;
        stmt.bind(3, category.section.as_str())?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        Ok(())
    }

    /// Updates the category and all references.
    fn category_update(&self, id: &str, category: &DBCategory) -> api::Result<()> {
        let transaction = self.db().transaction()?;
        // Update category
        let mut stmt = self.db().prepare(UPDATE)?;
        stmt.bind(1, category.id.as_str())?;
        stmt.bind(2, category.name.as_str())?;
        stmt.bind(3, category.section.as_str())?;
        stmt.bind(4, id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }

        if id != category.id {
            // Update category ids of related media
            let mut stmt = self.db().prepare(UPDATE_MEDIA)?;
            stmt.bind(1, category.id.as_str())?;
            stmt.bind(2, id)?;
            if stmt.next()? != sqlite::State::Done {
                return Err(api::Error::SQLError);
            }
        }

        transaction.commit()?;
        Ok(())
    }

    /// Removes the category, assuming it is not referenced anywhere.
    fn category_delete(&self, id: &str) -> api::Result<()> {
        let mut stmt = self.db().prepare(DELETE)?;
        stmt.bind(1, id)?;
        if stmt.next()? != sqlite::State::Done {
            return Err(api::Error::SQLError);
        }
        Ok(())
    }

    /// Returns the number of books in this category.
    fn category_references(&self, id: &str) -> api::Result<i64> {
        let mut stmt = self.db().prepare(REFERENCED)?;
        stmt.bind(1, id)?;
        if stmt.next()? != sqlite::State::Row {
            return Err(api::Error::SQLError);
        }
        Ok(stmt.read(0)?)
    }
}