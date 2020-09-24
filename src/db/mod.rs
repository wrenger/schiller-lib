use std::fmt;
use std::iter::FromIterator;
use std::path::PathBuf;

use gdnative::prelude::*;

mod raw;

use crate::api;

const QUERY_MEDIA_LIST: &str = r#"
select
id,
isbn,
title,
publisher,
year,
costs,
note,
borrowable,
category,
ifnull(group_concat(author.name),''),
borrower,
deadline,
reservation
from medium
left join author on author.medium=id
where title like '%'||?||'%'
group by id
"#;

const QUERY_USER_LIST: &str = r#"
select
account,
forename,
surname,
role,
may_borrow
from user
where account like '%'||?||'%'
"#;

pub struct Database {
    path: PathBuf,
    db: sqlite::Connection,
}

impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Database {{ {:?} }}", self.path)
    }
}

impl Database {
    pub fn new(path: &str) -> Result<Database, api::Error> {
        let path = PathBuf::from(path);
        if path.exists() {
            Ok(Database {
                db: sqlite::Connection::open(&path).map_err(|_| api::Error::FileOpenError)?,
                path,
            })
        } else {
            Err(api::Error::FileNotFound)
        }
    }

    pub fn search_media_basic(
        &self,
        text: &str,
    ) -> api::Result<Vec<Instance<api::Medium, Unique>>> {
        let mut stmt = self.db.prepare(QUERY_MEDIA_LIST)?;
        stmt.bind(1, text)?;

        let mut result = vec![];

        while stmt.next()? != sqlite::State::Done {
            let instance = api::Medium::new_instance();
            instance
                .map_mut(|medium, _| fill_medium(medium, &stmt))
                .unwrap()?;
            result.push(instance);
        }

        Ok(result)
    }

    pub fn search_user_basic(&self, text: &str) -> api::Result<Vec<Instance<api::User, Unique>>> {
        let mut stmt = self.db.prepare(QUERY_USER_LIST)?;
        stmt.bind(1, text)?;

        let mut result = vec![];

        while stmt.next()? != sqlite::State::Done {
            let instance = api::User::new_instance();
            instance
                .map_mut(|user, _| fill_user(user, &stmt))
                .unwrap()?;
            result.push(instance);
        }

        Ok(result)
    }

    pub fn update_medium(&self, previous_id: &str, medium: &api::Medium) -> api::Result<()> {
        godot_print!("update_medium {} -> {:?}", previous_id, medium);
        Ok(())
    }

    pub fn delete_medium(&self, id: &str) -> api::Result<()> {
        godot_print!("delete_medium {}", id);
        Ok(())
    }

    pub fn update_user(&self, previous_account: &str, user: &api::User) -> api::Result<()> {
        godot_print!("update_user {} -> {:?}", previous_account, user);
        Ok(())
    }

    pub fn delete_user(&self, account: &str) -> api::Result<()> {
        godot_print!("delete_user {}", account);
        Ok(())
    }
}

fn fill_user(user: &mut api::User, stmt: &sqlite::Statement) -> api::Result<()> {
    user.account = stmt.read::<String>(0)?.into();
    user.forename = stmt.read::<String>(1)?.into();
    user.surname = stmt.read::<String>(2)?.into();
    user.role = stmt.read::<String>(3)?.into();
    user.may_borrow = stmt.read::<i64>(4)? != 0;
    Ok(())
}

fn fill_medium(medium: &mut api::Medium, stmt: &sqlite::Statement) -> api::Result<()> {
    medium.id = stmt.read::<String>(0)?.into();
    medium.isbn = stmt.read::<String>(1)?.into();
    medium.title = stmt.read::<String>(2)?.into();
    medium.publisher = stmt.read::<String>(3)?.into();
    medium.year = stmt.read(4)?;
    medium.costs = stmt.read(5)?;
    medium.note = stmt.read::<String>(6)?.into();
    medium.borrowable = stmt.read::<i64>(7)? != 0;
    medium.category = stmt.read::<String>(8)?.into();
    medium.authors = StringArray::from_iter(stmt.read::<String>(9)?.split(',').map(|a| a.into()));
    medium.borrower = stmt.read::<String>(10)?.into();
    medium.deadline = stmt.read::<String>(11)?.into();
    medium.reservation = stmt.read::<String>(12)?.into();
    Ok(())
}
