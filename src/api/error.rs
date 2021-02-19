use gdnative::prelude::*;

/// The api compatible error type.
/// On the godot side there are specific error messages displayed for each of the error types.
///
/// More specific error messages are removed to be api compatible.
/// Those messages are logged however.
#[repr(i64)]
#[derive(Debug, Clone, Copy)]
pub enum Error {
    InvalidArguments,
    LogicError,
    NoProject,
    FileNotFound,
    FileOpenError,
    SQLError,
    NetworkError,
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
        godot_print!("SQLError: {}", e);
        Error::SQLError
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(e: std::convert::Infallible) -> Error {
        godot_print!("convert::Infallible: {}", e);
        Error::InvalidArguments
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        godot_print!("File Error: {:?}", e);
        Error::FileOpenError
    }
}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Error {
        godot_print!("Invalid Format {:?}", e);
        Error::InvalidFormat
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        godot_print!("Network Error: {:?}", e);
        Error::NetworkError
    }
}

impl From<roxmltree::Error> for Error {
    fn from(e: roxmltree::Error) -> Error {
        godot_print!("Invalid XML Format: {:?}", e);
        Error::InvalidFormat
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
