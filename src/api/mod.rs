use gdnative::prelude::*;

mod error;
pub use error::*;
mod medium;
pub use medium::Medium;
mod project;
mod user;
pub use user::User;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<project::Project>();
    handle.add_class::<Medium>();
    handle.add_class::<User>();
}

// Macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
