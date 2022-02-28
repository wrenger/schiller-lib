use std::convert::TryInto;

use gdnative::core_types::FromVariantError;
use gdnative::prelude::*;

/// The api compatible error type.
/// On the godot side there are specific error messages displayed for each of the error types.
///
/// More specific error messages are removed to be api compatible.
/// Those messages are logged however.
#[repr(i64)]
#[derive(Debug, Clone, Copy, num_enum::TryFromPrimitive)]
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

impl From<sqlite::Error> for Error {
    fn from(e: sqlite::Error) -> Error {
        godot_error!("SQL: {e}");
        Error::SQL
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(e: std::convert::Infallible) -> Error {
        godot_error!("convert::Infallible: {e:?}");
        Error::Arguments
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        godot_error!("File Error: {e:?}");
        Error::FileOpen
    }
}

impl From<roxmltree::Error> for Error {
    fn from(e: roxmltree::Error) -> Error {
        godot_error!("Invalid XML Format: {e:?}");
        Error::InvalidFormat
    }
}

impl gdnative::core_types::FromVariant for Error {
    fn from_variant(
        variant: &gdnative::core_types::Variant,
    ) -> std::result::Result<Self, FromVariantError> {
        i64::from_variant(variant)?
            .try_into()
            .map_err(|_| FromVariantError::Unspecified)
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
