use std::fmt;
use std::fs::File;
use std::io::{BufReader, Seek};
use std::path::Path;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::Database;
use crate::error::{Error, Result};

/// Version metadata, used for database migrations
#[derive(Serialize, Deserialize)]
struct DatabaseVersion {
    version: Version,
}

const MIN_VERSION: Version = Version(0, 9, 0);

pub fn import(path: &Path) -> Result<Database> {
    #[cfg(feature = "sqlite")]
    if path.extension() == Some(std::ffi::OsStr::new("db")) {
        tracing::warn!("Try importing old database");
        return from_db(path);
    }

    let mut file = File::open(path)?;

    let DatabaseVersion { version } = serde_json::from_reader(BufReader::new(&file))?;
    let pkg_version: Version = crate::PKG_VERSION.parse().unwrap();
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
fn from_db(file: &Path) -> Result<Database> {
    use tracing::{info, warn};

    let mut data = Database::default();

    let db = super::legacy::Database::open(file.into())?.0;
    info!("Transferring settings");
    data.settings = super::Settings::from(db.settings()?);

    info!("Transferring categories");
    for category in db.categories()? {
        let id = category.id.clone();
        if let Err(e) = data.categories.add(category.into()) {
            warn!("{e:?}: Failed adding category {id}");
        }
    }

    info!("Transferring users");
    for user in db.users()? {
        let account = user.account.clone();
        if let Err(e) = data.users.add(user.into()) {
            warn!("{e:?}: Failed adding user {account}");
        }
    }

    info!("Transferring books");
    for book in db.books()? {
        let id = book.id.clone();
        if let Err(e) = data.books.add(book.into(), &data.categories) {
            warn!("{e:?}: Failed adding book {id}");
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

impl<'de> Visitor<'de> for VersionVisitor {
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
    use crate::PKG_VERSION;

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
