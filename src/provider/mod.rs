pub mod book;
pub mod user;

#[repr(i64)]
#[derive(Debug, Clone, Copy)]
pub enum Error {
    InvalidConfig,
    InvalidInput,
    FileError,
    NetworkError,
    InvalidFormat,
    NothingFound,
}

impl gdnative::core_types::ToVariant for Error {
    fn to_variant(&self) -> gdnative::core_types::Variant {
        gdnative::core_types::Variant::from_i64(*self as i64)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        gdnative::godot_print!("File Error: {:?}", e);
        Error::FileError
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
        gdnative::godot_print!("Invalid Format: {:?}", e);
        Error::InvalidFormat
    }
}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Error {
        gdnative::godot_print!("Invalid Format {:?}", e);
        Error::InvalidFormat
    }
}

pub type Result<T> = std::result::Result<T, Error>;
