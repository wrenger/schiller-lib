use std::fmt;

use axum::Json;
use axum::response::IntoResponse;
use gluer::metadata;
use hyper::StatusCode;
use serde::Serialize;
use tracing::error;

/// The api compatible error type.
/// On the frontend there are specific error messages displayed for each of the error types.
///
/// More specific error messages are removed to be api compatible.
/// Those messages are logged however.
#[metadata]
#[repr(i64)]
#[derive(Debug, Clone, Copy, Serialize)]
pub enum Error {
    /// The user provided arguments are malformed
    Arguments,
    /// A file could not be found or opened
    FileOpen,
    /// Could not connect to server
    Network,
    /// Invalid file format
    InvalidFormat,
    /// No matching results
    NothingFound,
    /// Deletion not possible as the user is still referenced
    ReferencedUser,
    /// Deletion not possible as the category is still referenced
    ReferencedCategory,
    /// The book has invalid or missing fields
    InvalidBook,
    /// The user has invalid or missing fields
    InvalidUser,
    /// A user, book, or category already exists
    Duplicate,
    /// User may not borrow
    LendingUserMayNotBorrow,
    /// Book cannot be borrowed
    LendingBookNotBorrowable,
    /// Book is already borrowed
    LendingBookAlreadyBorrowed,
    /// Book cannot be reserved as the user already borrows it
    LendingBookAlreadyBorrowedByUser,
    /// The book cannot be reserved or returned as it is not borrowed
    LendingBookNotBorrowed,
    /// The book is already reserved
    LendingBookAlreadyReserved,
    /// The book is not reserved
    LendingBookNotReserved,
    /// The database version is too old
    UnsupportedProjectVersion,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(feature = "sqlite")]
#[allow(deprecated)]
impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        match e {
            rusqlite::Error::QueryReturnedNoRows => Self::NothingFound,
            _ => {
                error!("SQL: {e}");
                Self::InvalidFormat
            }
        }
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
        use std::io::ErrorKind;

        error!("File Error: {e}");
        match e.kind() {
            ErrorKind::ConnectionRefused
            | ErrorKind::ConnectionReset
            | ErrorKind::ConnectionAborted
            | ErrorKind::NotConnected
            | ErrorKind::AddrInUse
            | ErrorKind::AddrNotAvailable => Self::Network,
            _ => Self::FileOpen,
        }
    }
}
impl From<roxmltree::Error> for Error {
    fn from(e: roxmltree::Error) -> Self {
        error!("Invalid XML Format: {e:?}");
        Self::InvalidFormat
    }
}
impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Self {
        match e.into_kind() {
            csv::ErrorKind::Io(e) => Self::from(e),
            e => {
                error!("Invalid CSV Format: {e:?}");
                Self::InvalidFormat
            }
        }
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        error!("Invalid JSON Format: {e:?}");
        Self::InvalidFormat
    }
}
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        error!("Network request: {e:?}");
        Self::Network
    }
}
impl<ER: std::error::Error + 'static, T: oauth2::ErrorResponse + 'static>
    From<oauth2::RequestTokenError<ER, T>> for Error
{
    fn from(e: oauth2::RequestTokenError<ER, T>) -> Self {
        error!("OAUTH Failed: {e:?}");
        Self::Network
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Error::Arguments
            | Error::InvalidFormat
            | Error::ReferencedUser
            | Error::ReferencedCategory
            | Error::InvalidBook
            | Error::InvalidUser
            | Error::Duplicate
            | Error::LendingUserMayNotBorrow
            | Error::LendingBookNotBorrowable
            | Error::LendingBookAlreadyBorrowed
            | Error::LendingBookAlreadyBorrowedByUser
            | Error::LendingBookNotBorrowed
            | Error::LendingBookAlreadyReserved
            | Error::LendingBookNotReserved => StatusCode::BAD_REQUEST,
            Error::FileOpen | Error::NothingFound => StatusCode::NOT_FOUND,
            Error::UnsupportedProjectVersion => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Network => StatusCode::SERVICE_UNAVAILABLE,
        };
        (status, Json(self)).into_response()
    }
}

/// Result type using the api error.
#[metadata]
pub type Result<T> = std::result::Result<T, Error>;
