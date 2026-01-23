use std::fmt;
use std::fs::File;
use std::io::{BufReader, Seek};
use std::path::Path;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::Database;
use crate::error::{Error, Result};
use crate::server::UserConfig;
use crate::util::PKG_VERSION;

/// Version metadata, used for database migrations
#[derive(Serialize, Deserialize)]
struct DatabaseVersion {
    version: Version,
}

const MIN_VERSION: Version = Version(0, 9, 0);

pub fn import(path: &Path, #[allow(unused)] user: Option<&UserConfig>) -> Result<Database> {
    #[cfg(feature = "sqlite")]
    if path.extension() == Some(std::ffi::OsStr::new("db")) {
        tracing::warn!("Try importing old database");
        return from_db(path, user);
    }

    let mut file = File::open(path)?;

    let DatabaseVersion { version } = serde_json::from_reader(BufReader::new(&file))?;
    let pkg_version: Version = PKG_VERSION.parse().unwrap();
    if MIN_VERSION <= version && version <= pkg_version {
        file.rewind()?;
        // TODO: Migration routines
        Database::load(&file)
    } else {
        Err(Error::UnsupportedProjectVersion)
    }
}

#[cfg(feature = "sqlite")]
#[allow(deprecated)]
fn from_db(file: &Path, user_config: Option<&UserConfig>) -> Result<Database> {
    use std::collections::HashMap;

    use tracing::{error, info, warn};

    use crate::db::Category;
    use crate::provider;

    let mut data = Database::default();

    let db = super::legacy::Database::open(file)?.0;
    info!("Transferring settings");
    data.settings = super::Settings::from(db.settings()?);

    info!("Transferring categories");
    data.categories.add(Category {
        id: "none".into(),
        name: "None".into(),
        section: "None".into(),
    })?;

    for category in db.categories()? {
        let id = category.id.clone();
        if let Err(e) = data.categories.add(category.into()) {
            warn!("Category {id}: Failure {e:?}");
        }
    }

    let mut changed_accounts = HashMap::new();

    fn find_in_userfile(config: &Option<&UserConfig>, user: &super::User) -> Option<String> {
        if let Some(UserConfig { file, delimiter }) = config {
            match provider::user::load_all(file, *delimiter) {
                Ok(provided) => {
                    for found in provided {
                        if found.forename.to_lowercase() == user.forename.trim().to_lowercase()
                            && found.surname.to_lowercase() == user.surname.trim().to_lowercase()
                        {
                            return Some(found.account.clone());
                        }
                    }
                }
                Err(e) => error!("Error loading userfile: {e:?}"),
            }
        }
        None
    }

    info!("Transferring users");
    for user in db.users()? {
        let mut user = super::User::from(user);
        let account = user.account.clone();

        let mut changed = None;
        if !crate::mail::account_is_valid(&user.account) {
            warn!("User {account}: Invalid account");

            // Try to find in user file...
            if let Some(new_account) = find_in_userfile(&user_config, &user) {
                warn!("User {account}: Updated to {new_account} from userfile");
                changed = Some(new_account.clone());
                user.account = new_account;
            } else {
                // Or strip invalid characters
                let new = crate::util::convert_ascii_lower(&user.account, false);
                warn!("User {account}: Updated to {new} by normalization");
                changed = Some(new.clone());
                user.account = new;
            }
        } else if let Some(UserConfig { file, delimiter }) = &user_config
            && provider::user::get(file, *delimiter, &account).is_err()
        {
            warn!("User {account}: Not found in userfile");

            // Try to find in user file...
            if let Some(new_account) = find_in_userfile(&user_config, &user) {
                changed = Some(new_account.clone());
                warn!("User {account}: Updated to {new_account} from userfile");
                user.account = new_account;
            }
        }

        if let Err(e) = data.users.add(user.into()) {
            warn!("User {account}: Failure {e:?}");
            changed_accounts.insert(account, String::new());
        } else if let Some(changed) = changed {
            changed_accounts.insert(account, changed);
        }
    }

    info!("Transferring books");
    for book in db.books()? {
        let id = book.id.clone();
        let mut book = super::Book::from(book);
        if book.category.is_empty() || !data.categories.data.contains_key(&book.category) {
            warn!("Book {id}: Invalid category -> setting to 'none'");
            book.category = "none".into();
        }
        if let Some(borrower) = &mut book.borrower {
            if let Some(new_account) = changed_accounts.get(&borrower.user) {
                if new_account.is_empty() {
                    info!("Book {id}: Removing missing borrower {}", borrower.user);
                    book.borrower = None;
                } else {
                    info!("Book {id}: Updating borrower to {new_account}");
                    borrower.user = new_account.clone();
                }
            }
        }
        if let Some(reservation) = &mut book.reservation {
            if let Some(new_account) = changed_accounts.get(reservation) {
                if new_account.is_empty() {
                    info!("Book {id}: Removing missing reservation {reservation}");
                    book.reservation = None;
                } else {
                    info!("Book {id}: Updating reservation to {new_account}");
                    *reservation = new_account.clone();
                }
            }
        }

        if let Err(e) = data.books.add(book, &data.categories, &data.users) {
            warn!("Book {id}: Failure {e:?}");
        }
    }

    Ok(data)
}

/// Semantic Version: major, minor, patch
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub u8, pub u8, pub u8);

impl FromStr for Version {
    type Err = Error;
    fn from_str(version: &str) -> Result<Self> {
        let version_parts = version
            .splitn(3, '.')
            .map(str::parse)
            .collect::<std::result::Result<Vec<u8>, _>>()
            .map_err(|_| Error::UnsupportedProjectVersion)?;
        if let [major, minor, patch] = version_parts[..] {
            Ok(Version(major, minor, patch))
        } else {
            Err(Error::UnsupportedProjectVersion)
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{self}"))
    }
}

use serde::de::{self, Visitor};

struct VersionVisitor;

impl Visitor<'_> for VersionVisitor {
    type Value = Version;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a version string")
    }
    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse().map_err(E::custom)
    }
    fn visit_string<E>(self, value: String) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&value)
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Version, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(VersionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_parsing() {
        assert!("0.0.0".parse::<Version>().unwrap() == Version(0, 0, 0));
        assert!("0.1.0".parse::<Version>().unwrap() == Version(0, 1, 0));
        assert!("0.9.22".parse::<Version>().unwrap() == Version(0, 9, 22));
        assert!("10.9.22".parse::<Version>().unwrap() == Version(10, 9, 22));
        assert!("255.255.255".parse::<Version>().unwrap() == Version(255, 255, 255));

        assert!("10".parse::<Version>().is_err());
        assert!("1.2.3.4".parse::<Version>().is_err());
        assert!("0.-1".parse::<Version>().is_err());
        assert!("1.2.-2".parse::<Version>().is_err());
        assert!("..".parse::<Version>().is_err());

        assert!(PKG_VERSION.parse::<Version>().is_ok());
    }
}
