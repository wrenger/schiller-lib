use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::Serialize;
use tracing::error;

/// The api compatible error type.
/// On the godot side there are specific error messages displayed for each of the error types.
///
/// More specific error messages are removed to be api compatible.
/// Those messages are logged however.
#[repr(i64)]
#[derive(Debug, Clone, Copy, Serialize)]
pub enum Error {
    Arguments,
    Logic,
    FileNotFound,
    FileOpen,
    SQL,
    Network,
    InvalidFormat,
    NothingFound,
    // Specific errors
    InvalidBook,
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
        match e {
            rusqlite::Error::QueryReturnedNoRows => Self::NothingFound,
            _ => {
                error!("SQL: {e}");
                Self::SQL
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
            ErrorKind::NotFound => Self::FileNotFound,
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
            | Error::Logic
            | Error::InvalidFormat
            | Error::InvalidBook
            | Error::InvalidUser
            | Error::LendingUserMayNotBorrow
            | Error::LendingBookNotBorrowable
            | Error::LendingBookAlreadyBorrowed
            | Error::LendingBookAlreadyBorrowedByUser
            | Error::LendingBookNotBorrowed
            | Error::LendingBookAlreadyReserved => StatusCode::BAD_REQUEST,
            Error::FileNotFound | Error::NothingFound => StatusCode::NOT_FOUND,
            Error::FileOpen | Error::SQL | Error::UnsupportedProjectVersion => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Error::Network => StatusCode::SERVICE_UNAVAILABLE,
        };
        (status, Json(self)).into_response()
    }
}

/// Result type using the api error.
pub type Result<T> = std::result::Result<T, Error>;
