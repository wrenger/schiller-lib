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
}

impl From<sqlite::Error> for Error {
    fn from(e: sqlite::Error) -> Error {
        godot_print!("SQLError: {}", e);
        Error::SQLError
    }
}

impl ToVariant for Error {
    fn to_variant(&self) -> Variant {
        Variant::from_i64(*self as i64)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
