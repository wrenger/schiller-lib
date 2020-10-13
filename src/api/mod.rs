use gdnative::prelude::*;

mod error;
pub use error::*;

mod project;
mod date;
mod debug;

use project::Project;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<Project>();
    handle.add_class::<date::Date>();
}

// Macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
