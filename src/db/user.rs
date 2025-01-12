use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use gluer::metadata;
use serde::{Deserialize, Serialize};

use super::Books;
use crate::db::sorted::Sorted;
use crate::error::{Error, Result};
use crate::mail::account_is_valid;

/// Data object for a user.
#[metadata]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(default)]
pub struct User {
    pub account: String,
    pub forename: String,
    pub surname: String,
    #[meta(optional)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub role: String,
    #[meta(optional)]
    #[serde(skip_serializing_if = "Clone::clone")] // <- skip if true
    pub may_borrow: bool,
}

impl Default for User {
    fn default() -> Self {
        Self {
            account: Default::default(),
            forename: Default::default(),
            surname: Default::default(),
            role: Default::default(),
            may_borrow: true,
        }
    }
}

impl User {
    fn validate(&mut self) -> bool {
        self.account = self.account.trim().to_string();
        self.forename = self.forename.trim().to_string();
        self.surname = self.surname.trim().to_string();
        self.role = self.role.trim().to_string();
        account_is_valid(&self.account) && !self.forename.is_empty() && !self.surname.is_empty()
    }
}

/// Parameters for the normal search
#[metadata]
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct UserSearch {
    pub query: String,
    pub may_borrow: Option<bool>,
    pub offset: usize,
    pub limit: usize,
}

impl Default for UserSearch {
    fn default() -> Self {
        Self {
            query: Default::default(),
            may_borrow: None,
            offset: 0,
            limit: 100,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Users {
    #[serde(flatten)]
    pub data: BTreeMap<String, User>,
}

impl Users {
    pub fn fetch(&self, account: &str) -> Result<User> {
        let account = account.trim();
        if account.is_empty() {
            return Err(Error::Arguments);
        }
        self.data.get(account).cloned().ok_or(Error::NothingFound)
    }

    pub fn add(&mut self, mut user: User) -> Result<User> {
        if !user.validate() {
            return Err(Error::InvalidUser);
        }

        match self.data.entry(user.account.clone()) {
            Entry::Vacant(v) => {
                v.insert(user.clone());
                Ok(user)
            }
            _ => Err(Error::InvalidUser),
        }
    }

    pub fn update(&mut self, account: &str, mut user: User, books: &mut Books) -> Result<User> {
        let account = account.trim();
        if account.is_empty() || !user.validate() {
            return Err(Error::InvalidUser);
        }

        if account == user.account {
            if let Some(entry) = self.data.get_mut(account) {
                *entry = user.clone();
                return Ok(user);
            }
        } else if self.data.contains_key(account) {
            return match self.data.entry(user.account.clone()) {
                Entry::Vacant(v) => {
                    v.insert(user.clone());
                    books.update_user(account, &user.account)?;
                    self.data.remove(account);
                    Ok(user)
                }
                _ => Err(Error::InvalidUser),
            };
        }

        Err(Error::NothingFound)
    }

    pub fn delete(&mut self, account: &str, books: &Books) -> Result<()> {
        let account = account.trim();
        if account.is_empty() {
            return Err(Error::Arguments);
        }
        if books.is_user_referenced(account) {
            return Err(Error::ReferencedUser);
        }

        self.data
            .remove(account)
            .map(|_| ())
            .ok_or(Error::NothingFound)
    }

    /// Performes a simple user search with the given `text`.
    pub fn search(&self, search: &UserSearch) -> Result<(usize, Vec<User>)> {
        let mut results = Sorted::new(|a: &(usize, &User), b: &(usize, &User)| {
            a.0.cmp(&b.0)
                .reverse()
                .then_with(|| a.1.account.cmp(&b.1.account))
        });

        let query = search.query.trim().to_lowercase();
        let keywords = query.split_whitespace().collect::<Vec<_>>();

        // just a very basic brute-force search
        'users: for user in self.data.values() {
            match search.may_borrow {
                Some(b) if b != user.may_borrow => continue,
                _ => {}
            }

            if query.is_empty() {
                results.push((0, user));
                continue;
            }
            let account = user.account.to_lowercase();
            if query == account {
                results.push((100, user));
                continue;
            }

            let mut score = 0;
            for keyword in &keywords {
                if account.starts_with(keyword) {
                    score += 3;
                } else if account.contains(keyword) {
                    score += 2;
                } else if user.forename.to_lowercase().contains(keyword)
                    || user.surname.to_lowercase().contains(keyword)
                    || user.role.to_lowercase().contains(keyword)
                {
                    score += 1;
                } else {
                    // no match -> skip this user
                    continue 'users;
                }
            }
            if score > 0 {
                results.push((score, user));
            }
        }

        let total = results.len();
        let books = results
            .into_iter()
            .skip(search.offset)
            .take(search.limit)
            .map(|b| b.1.clone())
            .collect();
        Ok((total, books))
    }

    /// Deletes the roles from all users and inserts the new roles.
    ///
    /// The roles of all users not contained in the given list are cleared.
    pub fn update_roles(&mut self, users: &[(String, String)]) -> Result<()> {
        for user in self.data.values_mut() {
            user.role.clear();
        }

        for (account, role) in users {
            let account = account.trim();
            if !account.is_empty() {
                if let Some(entry) = self.data.get_mut(account) {
                    entry.role = role.clone();
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn add_update_remove_users() {
        let mut db = Database::default();

        let user = User {
            account: "foo.bar".into(),
            forename: "Foo".into(),
            surname: "Bar".into(),
            role: "Demo".into(),
            may_borrow: true,
        };
        db.users.add(user.clone()).unwrap();

        let (count, users) = db
            .users
            .search(&UserSearch {
                query: "".to_owned(),
                may_borrow: None,
                offset: 0,
                limit: 100,
            })
            .unwrap();
        assert_eq!(count, 1);
        assert_eq!(users[0], user);

        db.users
            .update(
                &user.account,
                User {
                    role: "Teacher".into(),
                    ..user.clone()
                },
                &mut db.books,
            )
            .unwrap();
        let (count, users) = db
            .users
            .search(&UserSearch {
                query: "".to_owned(),
                may_borrow: None,
                offset: 0,
                limit: 100,
            })
            .unwrap();
        assert_eq!(count, 1);
        assert_eq!(users[0].role, "Teacher");

        db.users.delete(&user.account, &mut db.books).unwrap();
        let (count, _) = db
            .users
            .search(&UserSearch {
                query: "".to_owned(),
                may_borrow: None,
                offset: 0,
                limit: 100,
            })
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn update_user_roles() {
        let mut db = Database::default();

        let mut user1 = User {
            account: "foo.bar".into(),
            forename: "Foo".into(),
            surname: "Bar".into(),
            role: "Demo".into(),
            may_borrow: true,
        };
        let mut user2 = User {
            account: "baz.boz".into(),
            forename: "Baz".into(),
            surname: "Boz".into(),
            role: "Demo".into(),
            may_borrow: true,
        };
        db.users.add(user1.clone()).unwrap();
        db.users.add(user2.clone()).unwrap();

        let (count, users) = db
            .users
            .search(&UserSearch {
                query: "".to_owned(),
                may_borrow: None,
                offset: 0,
                limit: 100,
            })
            .unwrap();
        assert_eq!(count, 2);
        assert_eq!(users[0], user2);
        assert_eq!(users[1], user1);

        db.users
            .update_roles(&[("foo.bar".into(), "Teacher".into())])
            .unwrap();

        user1.role = "Teacher".into();
        user2.role = "".into();

        let (count, users) = db
            .users
            .search(&UserSearch {
                query: "".to_owned(),
                may_borrow: None,
                offset: 0,
                limit: 100,
            })
            .unwrap();
        assert_eq!(count, 2);
        assert_eq!(users[0], user2);
        assert_eq!(users[1], user1);
    }
}
