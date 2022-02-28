
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const PKG_LICENSE: &str = env!("CARGO_PKG_LICENSE");

#[cfg(not(test))]
macro_rules! error {
    ($($args:tt)*) => {
        gdnative::godot_error!($($args)*)
    };
}
#[cfg(test)]
macro_rules! error {
    ($($args:tt)*) => {
        println!($($args)*)
    };
}

#[cfg(not(test))]
macro_rules! info {
    ($($args:tt)*) => {
        gdnative::godot_print!($($args)*)
    };
}
#[cfg(test)]
macro_rules! info {
    ($($args:tt)*) => {
        println!($($args)*);
    };
}

mod api;
mod db;
mod provider;
mod isbn;
mod mail;
