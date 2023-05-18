use gdnative::core_types::FromVariantError;
use gdnative::prelude::*;

/// The api compatible error type.
/// On the godot side there are specific error messages displayed for each of the error types.
///
/// More specific error messages are removed to be api compatible.
/// Those messages are logged however.
#[repr(i64)]
#[derive(Debug, Clone, Copy)]
pub enum Error {
    Arguments,
    Logic,
    NoProject,
    FileNotFound,
    FileOpen,
    SQL,
    Network,
    InvalidFormat,
    NothingFound,
    // Specific errors
    InvalidBook,
    InvalidISBN,
    InvalidUser,
    // Lending errors
    LendingUserMayNotBorrow,
    LendingBookNotBorrowable,
    LendingBookAlreadyBorrowed,
    LendingBookAlreadyBorrowedByUser,
    LendingBookNotBorrowed,
    LendingBookAlreadyReserved,
    // Migration
    UnsupportedProjectVersion,
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        error!("SQL: {e}");
        Self::SQL
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(e: std::convert::Infallible) -> Self {
        error!("convert::Infallible: {e:?}");
        Self::Arguments
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        error!("File Error: {e:?}");
        Self::FileOpen
    }
}

impl From<roxmltree::Error> for Error {
    fn from(e: roxmltree::Error) -> Self {
        error!("Invalid XML Format: {e:?}");
        Self::InvalidFormat
    }
}

impl gdnative::core_types::FromVariant for Error {
    fn from_variant(
        variant: &gdnative::core_types::Variant,
    ) -> std::result::Result<Self, FromVariantError> {
        let val = i64::from_variant(variant)?;
        if 0 <= val && val <= Error::UnsupportedProjectVersion as i64 {
            Ok(unsafe { std::mem::transmute(val) })
        } else {
            Err(FromVariantError::Unspecified)
        }
    }
}

impl ToVariant for Error {
    #[inline]
    fn to_variant(&self) -> Variant {
        (*self as i64).to_variant()
    }
}

/// Result type using the api error.
pub type Result<T> = std::result::Result<T, Error>;
