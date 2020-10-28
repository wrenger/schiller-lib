use gdnative::prelude::*;

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
        gdnative::godot_print!("File Error: {:?}", e);
        Error::FileOpenError
    }
}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Error {
        gdnative::godot_print!("Invalid Format {:?}", e);
        Error::InvalidFormat
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        gdnative::godot_print!("Network Error: {:?}", e);
        Error::NetworkError
    }
}

impl From<roxmltree::Error> for Error {
    fn from(e: roxmltree::Error) -> Error {
        gdnative::godot_print!("Invalid XML Format: {:?}", e);
        Error::InvalidFormat
    }
}

impl ToVariant for Error {
    #[inline]
    fn to_variant(&self) -> Variant {
        (*self as i64).to_variant()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
