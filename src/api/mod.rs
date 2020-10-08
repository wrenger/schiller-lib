use gdnative::prelude::*;

mod error;
pub use error::*;

mod category;
mod medium;
mod project;
mod settings;
mod user;

use category::Category;
use medium::Medium;
use project::Project;
use settings::Settings;
use user::User;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<Category>();
    handle.add_class::<Medium>();
    handle.add_class::<Project>();
    handle.add_class::<Settings>();
    handle.add_class::<User>();
}

// Macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
